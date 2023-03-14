import {createStore, StateMachineProvider} from "little-state-machine"
import React from "react"
import ReactDOM from "react-dom/client"
import {createBrowserRouter, RouterProvider} from "react-router-dom"
import "./index.css"
import SelectMode from "./routes/select-mode"
import Root from "./routes/root"
import SelectCriteria, {
  loader as criteriaLoader,
} from "./routes/select-criteria"
import {FormStage} from "./types/types"
import SelectMarkers, {loader as markerLoader} from "./routes/select-markers"
import VideoOptions from "./routes/video-options"
import Progress from "./routes/progress"
import {nanoid} from "nanoid"
import {loader as rootLoader} from "./routes/root"
import ConfigPage from "./routes/config"
import PreviewClips, {loader as clipLoader} from "./routes/clips"

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    loader: rootLoader,
    children: [
      {
        index: true,
        element: <SelectMode />,
      },
      {
        path: "/select-criteria",
        element: <SelectCriteria />,
        loader: criteriaLoader,
      },
      {
        path: "/select-markers",
        element: <SelectMarkers />,
        loader: markerLoader,
      },
      {
        path: "/clips",
        element: <PreviewClips />,
        loader: clipLoader,
      },
      {
        path: "/video-options",
        element: <VideoOptions />,
      },
      {
        path: "/progress",
        element: <Progress />,
      },
    ],
  },
  {
    path: "/config",
    element: <ConfigPage />,
  },
])

createStore(
  {
    data: {
      stage: FormStage.SelectMode,
      id: nanoid(8),
    },
  },
  {
    name: "form-state",
  }
)

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <StateMachineProvider>
      <RouterProvider router={router} />
    </StateMachineProvider>
  </React.StrictMode>
)
