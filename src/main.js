const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

let tagInput;
window.addEventListener("DOMContentLoaded", () => {
    tagBar = document.querySelector("#tagbar");
    tagInput = document.querySelector("#tag-input");
    fileItemContainer = document.querySelector("#note-container");
    document.querySelector("#search-form").addEventListener("submit", async (e) => {
        e.preventDefault();
        //add tag
        await addTag(tagInput.value);
    });
});

let fileItemContainer;
async function search() {
    console.log("test: search with tags: ", Array.from(tags));
    
    const noteItems = invoke("find_notes_by_tags", { tags: Array.from(tags) })
        .then((notes) => {
            console.log('(search) searched note items: ', notes);
            return notes;
        })
        .catch(err => console.log(err));
}

let tags = new Set();
async function addTag(newTag) {
    if (!newTag) {
        await search();
        return;
    }

    if (tags.has(newTag)) return;

    tags.add(newTag);
    renderTagbar();
    
    tagInput.value = "";
    await search();
}

let tagBar;
function renderTagbar() {
    tagBar.innerHTML = "";
    Array.from(tags).forEach((tag) => {
        const tagItem = document.createElement("div");
        tagItem.className = "tag-item";
        tagItem.textContent = tag;
        tagBar.appendChild(tagItem);
    });
}