export interface Tag {
  name: string
  id: string
  count: number
}

export interface Performer {
  name: string
  id: string
  sceneCount: number
  imageUrl?: string
}

export enum FormStage {
  SelectMode = 1,
  SelectCriteria = 2,
  SelectMarkers = 3,
  PreviewClips = 4,
  VideoOptions = 5,
  Wait = 6,
}

export interface SelectedMarker {
  id: string
  duration?: number
}

export interface FormState {
  selectMode?: "tags" | "performers"
  selectedIds?: string[]
  clipOrder?: "random" | "scene-order"
  clipDuration?: number
  outputResolution?: "720" | "1080" | "4K"
  outputFps?: number
  selectedMarkers?: SelectedMarker[]
  markers?: unknown[]
  stage: FormStage
  id: string
}
