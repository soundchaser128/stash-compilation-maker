import {LoaderFunction, useLoaderData} from "react-router-dom"

export const loader: LoaderFunction = () => {
  return null
}

function PreviewClips() {
  const data = useLoaderData()
  return <p>hi</p>
}

export default PreviewClips
