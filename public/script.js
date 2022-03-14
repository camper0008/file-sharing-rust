const uploadInput = document.getElementById("file-upload");
const uploadLabel = document.getElementById("file-upload-label");
const clearInput = document.getElementById("file-clear");
const fileContainer = document.getElementById("file-container");
const MAX_FILENAME_LENGTH = 48;

const shortenName = (name) => {
    if (name.length < MAX_FILENAME_LENGTH) {
        return name;
    } else {
        const shortened = name.slice(MAX_FILENAME_LENGTH - 3);
        return shortened + "...";
    }
}

const createFileAnchorElement = (fileName) => {
    const anchor = document.createElement('a');
    anchor.innerText = shortenName(fileName);

    anchor.setAttribute('href', `/api/files/${fileName}`);
    anchor.setAttribute('download', '');
    anchor.setAttribute('class', 'text interact file');

    if (fileName.length > MAX_FILENAME_LENGTH)
        anchor.setAttribute('title', fileName);

    return anchor;
}

const updateFileView = async () => {
    const res = await (await fetch('/api/filelist')).json()
    const files = res.files;
    const children = files.map(createFileAnchorElement);
    fileContainer.replaceChildren(...children);
}

const uploadInputChanged = () => {
    uploadLabel.textContent = `select files (${uploadInput.files.length} selected)`
}

const clearInputClick = async () => {
    await fetch('/api/clear', {
        method: "POST",
    });
    updateFileView();
}

const main = async () => {
    uploadInput.addEventListener('change', () => uploadInputChanged());
    clearInput.addEventListener('click', () => clearInputClick());
    uploadInputChanged();
    updateFileView();
}

main();
