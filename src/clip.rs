use std::cmp::Reverse;

use crate::{http::CreateVideoBody, stash_api::GqlMarker, util};
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClipOrder {
    Random,
    SceneOrder,
}

pub struct ClipSettings {
    pub order: ClipOrder,
    pub max_clip_length: u32,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub marker_id: String,
    pub range: (u32, u32),
    pub marker_index: usize,
}

pub struct MarkerWithClips {
    pub marker: GqlMarker,
    pub clips: Vec<Clip>,
}

pub fn get_time_range(marker: &GqlMarker, max_duration: Option<u32>) -> (u32, Option<u32>) {
    let start = marker.seconds;
    let next_marker = marker
        .scene
        .scene_markers
        .iter()
        .find(|m| m.seconds > marker.seconds);
    if let Some(max_duration) = max_duration {
        (start as u32, Some(start as u32 + max_duration))
    } else if let Some(next) = next_marker {
        (start as u32, Some(next.seconds as u32))
    } else {
        (start as u32, None)
    }
}

pub fn get_clips(
    marker: &GqlMarker,
    settings: &ClipSettings,
    max_duration: Option<u32>,
    rng: &mut StdRng,
) -> MarkerWithClips {
    let clip_lengths = [
        (settings.max_clip_length / 2).max(2),
        (settings.max_clip_length / 3).max(2),
        (settings.max_clip_length / 4).max(2),
    ];

    let (start, end) = get_time_range(marker, max_duration);
    let end = end.unwrap_or(start + settings.max_clip_length);

    let mut index = 0;
    let mut offset = start;
    let mut clips = vec![];
    while offset < end {
        let duration = clip_lengths.choose(rng).unwrap();
        clips.push(Clip {
            marker_id: marker.id.clone(),
            range: (offset, *duration),
            marker_index: index,
        });
        offset += duration;
        index += 1;
    }

    MarkerWithClips {
        clips,
        marker: marker.clone(),
    }
}

pub fn get_all_clips(output: &CreateVideoBody) -> Vec<MarkerWithClips> {
    let mut rng = util::create_seeded_rng();
    let settings = ClipSettings {
        max_clip_length: output.clip_duration,
        order: output.clip_order,
    };
    output
        .markers
        .iter()
        .filter(|m| output.selected_markers.iter().any(|c| c.id == m.id))
        .map(|marker| {
            let selected_marker = output
                .selected_markers
                .iter()
                .find(|c| c.id == marker.id)
                .unwrap();
            get_clips(marker, &settings, selected_marker.duration, &mut rng)
        })
        .collect()
}

pub fn compile_clips(clips: Vec<MarkerWithClips>, _order: ClipOrder) -> Vec<Clip> {
    let mut rng = util::create_seeded_rng();
    let mut clips: Vec<_> = clips
        .into_iter()
        .flat_map(|m| m.clips)
        .map(|c| (c, rng.gen::<u32>()))
        .collect();

    clips.sort_by_key(|(clip, random)| Reverse((clip.marker_index, *random)));
    clips.into_iter().map(|(clip, _)| clip).collect()
}
