use crate::{config::Config, http::FilterMode, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

use self::{
    find_markers_query::{
        CriterionModifier, FindFilterType, FindMarkersQueryFindSceneMarkersSceneMarkers,
        HierarchicalMultiCriterionInput, MultiCriterionInput, SceneMarkerFilterType,
    },
    find_performers_query::FindPerformersQueryFindPerformersPerformers as Performer,
    find_tags_query::FindTagsQueryFindTagsTags as Tag,
};

pub type GqlMarker = FindMarkersQueryFindSceneMarkersSceneMarkers;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/find_tags.graphql",
    response_derives = "Debug"
)]
pub struct FindTagsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/find_markers.graphql",
    response_derives = "Debug, Clone, Serialize"
)]
pub struct FindMarkersQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/find_performers.graphql",
    response_derives = "Debug"
)]
pub struct FindPerformersQuery;

pub struct Api {
    api_url: String,
    api_key: String,
    client: Client,
}

impl Api {
    pub fn new(url: &str, api_key: &str) -> Self {
        Api {
            api_url: url.into(),
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    pub fn from_config(config: &Config) -> Self {
        Self::new(&config.stash_url, &config.api_key)
    }

    pub async fn load_config() -> Result<Self> {
        let config = Config::get().await?;
        Ok(Self::new(&config.stash_url, &config.api_key))
    }

    pub async fn find_tags(&self) -> Result<Vec<Tag>> {
        let variables = find_tags_query::Variables {};
        let request_body = FindTagsQuery::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        tracing::debug!("url = '{url}', api key = '{}'", self.api_key);

        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        let response: Response<find_tags_query::ResponseData> = response.json().await?;
        let tags = response.data.unwrap().find_tags.tags;

        Ok(tags)
    }

    pub async fn find_markers(&self, ids: Vec<String>, mode: FilterMode) -> Result<Vec<GqlMarker>> {
        let mut scene_filter = SceneMarkerFilterType {
            created_at: None,
            scene_created_at: None,
            scene_updated_at: None,
            updated_at: None,
            performers: None,
            scene_date: None,
            scene_tags: None,
            tag_id: None,
            tags: None,
        };

        match mode {
            FilterMode::Performers => {
                scene_filter.performers = Some(MultiCriterionInput {
                    modifier: CriterionModifier::INCLUDES,
                    value: Some(ids),
                });
            }
            FilterMode::Tags => {
                scene_filter.tags = Some(HierarchicalMultiCriterionInput {
                    depth: None,
                    modifier: CriterionModifier::INCLUDES,
                    value: Some(ids),
                });
            }
        }
        let variables = find_markers_query::Variables {
            filter: Some(FindFilterType {
                per_page: Some(-1),
                page: None,
                q: None,
                sort: None,
                direction: None,
            }),
            scene_marker_filter: Some(scene_filter),
        };

        let request_body = FindMarkersQuery::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;

        let response: Response<find_markers_query::ResponseData> = response.json().await?;
        let markers = response.data.unwrap();
        Ok(markers.find_scene_markers.scene_markers)
    }

    pub async fn find_performers(&self) -> Result<Vec<Performer>> {
        let variables = find_performers_query::Variables {};
        let request_body = FindPerformersQuery::build_query(variables);
        let url = format!("{}/graphql", self.api_url);
        let response = self
            .client
            .post(url)
            .json(&request_body)
            .header("ApiKey", &self.api_key)
            .send()
            .await?
            .error_for_status()?;

        let response: Response<find_performers_query::ResponseData> = response.json().await?;
        let performers = response.data.unwrap();
        Ok(performers.find_performers.performers)
    }
}
