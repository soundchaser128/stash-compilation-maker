import {useForm} from "react-hook-form"
import {useNavigate} from "react-router-dom"

interface Inputs {
  stashUrl: string
  apiKey: string
}

function ConfigPage() {
  const {watch, register, handleSubmit} = useForm<Inputs>()
  const urlValue = watch("stashUrl") || "http://localhost:9999"
  const settingsPage = `${urlValue}/settings?tab=security`
  const navigate = useNavigate()

  const onSubmit = async (inputs: Inputs) => {
    const response = await fetch("/api/config", {
      method: "POST",
      body: JSON.stringify(inputs),
      headers: {"content-type": "application/json"},
    })
    if (response.ok) {
      navigate("/")
    }
  }

  return (
    <main className="container ml-auto mr-auto">
      <section className="py-4 flex flex-col">
        <h1 className="text-4xl font-bold mb-4 text-center">
          Configuration setup
        </h1>
        <form onSubmit={handleSubmit(onSubmit)} className="max-w-lg w-full">
          <div className="form-control">
            <label className="label">
              <span className="label-text">URL of your Stash instance:</span>
            </label>
            <input
              type="url"
              placeholder="Example: http://localhost:9999"
              className="input input-bordered"
              defaultValue="http://localhost:9999"
              required
              {...register("stashUrl", {required: true})}
            />
          </div>

          <div className="form-control">
            <label className="label">
              <span className="label-text">API key:</span>
            </label>
            <input
              type="text"
              placeholder="eyJhbGc..."
              className="input input-bordered"
              required
              {...register("apiKey", {required: true})}
            />
            <label className="label">
              <span className="label-text-alt">
                Navigate to{" "}
                <a className="link" href={settingsPage} target="_blank">
                  {settingsPage}
                </a>{" "}
                to retrieve your API key.
              </span>
            </label>
          </div>
          <button type="submit" className="btn btn-success">
            Submit
          </button>
        </form>
      </section>
    </main>
  )
}

export default ConfigPage
