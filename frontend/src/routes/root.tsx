import clsx from "clsx"
import {useStateMachine} from "little-state-machine"
import {useEffect} from "react"
import {
  LoaderFunction,
  Outlet,
  useLoaderData,
  useNavigate,
  useNavigation,
} from "react-router-dom"
import {FormStage} from "../types/types"
import {resetForm} from "./actions"

export const loader: LoaderFunction = async () => {
  const response = await fetch("/api/config")
  return response.ok
}

export default function Root() {
  const {state, actions} = useStateMachine({resetForm})
  const stage = state.data.stage
  const navigate = useNavigate()
  const navigation = useNavigation()
  const isLoading = navigation.state === "loading"
  const configExists = useLoaderData() as boolean

  useEffect(() => {
    if (!configExists) {
      navigate("/config")
    }
  }, [configExists])

  const onReset = () => {
    if (
      confirm(
        "Are you sure you want to reset the form and return to the start?"
      )
    ) {
      actions.resetForm()
      navigate("/")
    }
  }

  return (
    <div
      className={clsx(
        "min-h-screen flex flex-col justify-between transition",
        isLoading && "opacity-25"
      )}
    >
      <main className="container ml-auto mr-auto">
        <section className="py-4 flex flex-col">
          <h1 className="text-4xl font-bold mb-4 text-center">
            Stash Compilation Generator
          </h1>
          <div className="self-center flex gap-2 mb-4">
            <button onClick={onReset} className="btn btn-sm btn-error">
              Reset
            </button>
          </div>
          <ul className="steps mb-4">
            <li className="step step-primary">Choose mode</li>
            <li
              className={clsx(
                "step",
                stage >= FormStage.SelectCriteria && "step-primary"
              )}
            >
              Select criteria
            </li>
            <li
              className={clsx(
                "step",
                stage >= FormStage.SelectMarkers && "step-primary"
              )}
            >
              Select markers
            </li>
            <li
              className={clsx(
                "step",
                stage >= FormStage.PreviewClips && "step-primary"
              )}
            >
              Preview clips
            </li>
            <li
              className={clsx(
                "step",
                stage >= FormStage.VideoOptions && "step-primary"
              )}
            >
              Select video options
            </li>
            <li
              className={clsx(
                "step",
                stage >= FormStage.Wait && "step-primary"
              )}
            >
              Wait for video
            </li>
          </ul>
          <Outlet />
        </section>
      </main>
      <footer className="w-full text-center text-sm font-light flex flex-col gap-1 my-4">
        <p>
          Made by{" "}
          <a href="https://soundchaser128.xyz" className="link">
            soundchaser128
          </a>
        </p>
        <p>
          This project is open source and available on{" "}
          <a
            className="link"
            href="https://github.com/soundchaser128/stash-compilation-maker"
          >
            GitHub
          </a>
        </p>
      </footer>
    </div>
  )
}
