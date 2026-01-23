// due to https://github.com/DioxusLabs/dioxus/issues/2405
function focusElement() {
    const element = document.getElementById('game-container');
    
    if (element) {
        element.setAttribute('tabindex', '-1');
        element.focus();
    }
}

function setupAutoFocus() {
    const element = document.getElementById('game-container');
    
    if (!element) return;
    
    // Initial focus
    focusElement();
    
    // Re-focus when window regains focus
    window.addEventListener('focus', () => {
        focusElement();
    });
    
    // Re-focus if user clicks elsewhere
    document.addEventListener('click', (event) => {
        if (event.target !== element && !element.contains(event.target)) {
            focusElement();
        }
    });
}

setupAutoFocus();