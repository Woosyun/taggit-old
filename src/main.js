const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

listen('tauri://file-drop', event => {
    console.log(event.payload);
    invoke("insert_notes_with_tags", { files: event.payload, tags: Array.from(tags) });
});
listen('tauri://file-drop-hover', e => {
    console.log("something draged over!!");
})

let searchInput;
window.addEventListener("DOMContentLoaded", () => {
    tagbar = document.querySelector("#tagbar");
    searchInput = document.querySelector("#search-input");
    noteItemContainer = document.querySelector("#note-item-container");
    noteOptionPopover = document.querySelector("#note-option-popover");
    document.querySelector("#searchbar").addEventListener("submit", handleSubmit);
});

const defaultTag = 'item';
let tags = new Set([defaultTag]);
async function handleSubmit(e) {
    e.preventDefault();

    const newTag = searchInput.value;
    if (!newTag) {
        await search();
        return;
    };

    if (tags.has(newTag)) return;
    
    tags.add(newTag);
    search();
    
    searchInput.value = "";
    renderTagbar();
}

let tagbar;
function renderTagbar() {
    tagbar.innerHTML = "";
    Array.from(tags).forEach((tag) => {
        if (tag === defaultTag) return;
        
        const tagItem = document.createElement("div");
        tagItem.className = "badge";
        tagItem.textContent = tag;
        tagItem.addEventListener("click", () => {
            if (tag !== defaultTag) tags.delete(tag);
            renderTagbar();
            search();
        });
        
        tagbar.appendChild(tagItem);
    });
}

let noteItemContainer;
async function search() {
    console.log("test: search with tags: ", Array.from(tags));
    noteItemContainer.innerHTML = "";
    
    invoke("find_notes_by_tags", { tags: Array.from(tags) })
        .then((notes) => {
            console.log('(search) searched note items: ', notes);
            notes.forEach(renderNoteItem);
        })
        .catch(err => console.log(err));
}

let noteOptionPopover;
async function renderNoteItem(note) {
    //get tags
    
    const noteItem = document.createElement("div");
    noteItem.className = "note-item";
    noteItem.innerHTML = `
        <div>
            <p>${note.name}</p>
            <span>${note.lastModified}</span>
        </div>
    `;

    //add option button
    const optionButton = document.createElement("button");
    optionButton.popoverTargetElement = noteOptionPopover;
    optionButton.innerText = 'o';
    optionButton.addEventListener('click', function () {
        noteOptionPopover.innerHTML = `
            <p>${note.name}</p>
            <p>${note.path}</p>
            <span>${note.lastModified}</span>
            //render tags
        `;
    });
    noteItem.appendChild(optionButton);

    noteItemContainer.appendChild(noteItem);
}