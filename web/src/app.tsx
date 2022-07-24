import clsx from "clsx";
import { useState } from "preact/hooks";
import { constructUrl, createPaste, deletePaste } from "./service";

function Divider({ className }: { className?: string }) {
  return (
    <hr class={clsx("block h-[1px] bg-zinc-300 border-none", className)} />
  );
}

interface PasteStateSucceeded {
  type: "succeeded";
  deleteToken: string;
  slug: string;
  url: string;
}

interface PasteStateFailed {
  type: "failed";
  error: string;
}

interface PasteStateDeleted {
  type: "deleted";
}

type PasteState =
  | PasteStateSucceeded
  | PasteStateFailed
  | PasteStateDeleted
  | {
      type: "pending";
    };

function PasteStateVisualizer({
  state,
  onStateChange,
}: {
  state: PasteState;
  onStateChange: (state: PasteState) => void;
}) {
  const inner = (() => {
    switch (state.type) {
      case "succeeded":
        return (
          <>
            <h3 class="is-size-2">Pasted!</h3>
            <p>
              Your paste is now available at{" "}
              <a href={state.url} target="_blank" rel="nofollower noreferrer">
                {state.url}
              </a>
            </p>

            <button
              class="button is-danger mt-8 is-light"
              onClick={() => {
                deletePaste({
                  slug: state.slug,
                  deleteToken: state.deleteToken,
                })
                  .then(() => {
                    onStateChange({ type: "deleted" });
                  })
                  .catch((error) => {
                    alert("Failed to delete paste: " + error);
                  });
              }}
            >
              Delete Paste
            </button>
          </>
        );
      case "failed":
        return (
          <>
            <h3 class="is-size-2">Failed to paste</h3>
            <p>{state.error}</p>
          </>
        );
      case "deleted":
        return (
          <>
            <h3 class="is-size-2">Paste deleted</h3>
            <p>Your paste has been deleted. You can paste again if you want.</p>
          </>
        );
      case "pending":
        return null;
      default:
        throw new Error("Unknown state type");
    }
  })();

  return (
    inner && (
      <div>
        <Divider className="my-4" />
        {inner}
      </div>
    )
  );
}

export function App() {
  const [pasteState, setPasteState] = useState<PasteState>({ type: "pending" });

  return (
    <>
      <div class="container">
        <h1 class="is-size-3 mb-4">rocket.rs PasteBin!</h1>

        <Divider />

        <form
          action="/"
          method="post"
          onSubmit={(e) => {
            e.preventDefault();
            const form = e.target as HTMLFormElement;
            const content = form.content.value as string;
            createPaste({
              content,
            })
              .then((paste) => {
                console.log("Paste created", paste);
                setPasteState({
                  type: "succeeded",
                  deleteToken: paste.deleteToken,
                  slug: paste.slug,
                  url: constructUrl("/" + paste.slug).toString(),
                });
              })
              .catch((err) => {
                console.error("Failed to create paste", err);
                setPasteState({
                  type: "failed",
                  error: err.message,
                });
              });
          }}
        >
          <div class="field w-full">
            <label class="label">Code</label>
            <div class="control">
              <textarea name="content" class="textarea font-mono" rows={15} />
            </div>
          </div>
          <div class="field">
            <button class="button is-primary is-medium" type="submit">
              Paste!
            </button>
          </div>
        </form>

        <PasteStateVisualizer
          state={pasteState}
          onStateChange={setPasteState}
        />
      </div>
    </>
  );
}
