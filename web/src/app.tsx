import { useCallback, useState } from "preact/hooks";

export function App() {
  const [textContent, setTextContent] = useState<string>("");
  const onChange = useCallback((value: string) => {
    setTextContent(value);
  }, []);

  return (
    <div class="container">
      <h1 class="is-size-3 mb-4">rocket.rs PasteBin!</h1>
      <hr class="block h-[1px] bg-zinc-300 border-none" />
      <form action="/" method="post">
        <div class="field w-full">
          <label class="label">Code</label>
          <div class="control">
            <textarea
              class="textarea font-mono"
              rows={15}
              value={textContent}
              onChange={(e) => {
                onChange(e.currentTarget.value);
              }}
            />
          </div>
        </div>
        <div class="field">
          <button class="button is-primary is-medium" type="submit">
            Paste!
          </button>
        </div>
      </form>
    </div>
  );
}
