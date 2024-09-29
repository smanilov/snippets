function App() {
  let backendEndpoint = "http://127.0.0.1:11840/get_session";

  // Tailwind styles for the elements on the page.
  let style = {
    topLevel: "bg-gray-100 min-h-screen flex items-center justify-center",
    container: "bg-white p-8 rounded-lg shadow-lg w-full max-w-md",
    header: "text-2xl font-bold mb-6 text-center",
    label: "block text-sm font-medium text-gray-700",
    input:
      "mt-1 block w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm",
    button:
      "w-full bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-opacity-50",
  };
  return (
    <div class={style.topLevel}>
      <div class={style.container}>
        <h2 class={style.header}>Enter snippets</h2>

        <form action={backendEndpoint} method="POST">
          <div class="mb-4">
            <label for="access_key" class={style.label}>
              Access Key
            </label>
            <input
              type="text"
              id="access_key"
              name="access_key"
              placeholder="Your Access Key"
              required
              class={style.input}
            />
          </div>

          <div>
            <button type="submit" class={style.button}>
              Submit
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default App;
