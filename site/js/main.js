// Conway's Game of Life Background
let conwayGame = null;
let conwayEnabled = false;

function toggleLife() {
    conwayEnabled = !conwayEnabled;
    if (conwayEnabled) {
        startConway();
    } else {
        stopConway();
    }
}

function startConway() {
    if (conwayGame) return; // Already running
    
    const canvas = document.getElementById('snake-canvas');
    const ctx = canvas.getContext('2d');
    
    // Set canvas to full window size
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    
    const cellSize = 16;
    const cols = Math.floor(canvas.width / cellSize);
    const rows = Math.floor(canvas.height / cellSize);
    
    let grid = Array(rows).fill().map(() => Array(cols).fill(0));
    
    // Add multiple Gosper Guns across the screen
    function addGosperGun(startX, startY) {
        const pattern = [
            [24,0],[22,1],[24,1],[12,2],[13,2],[20,2],[21,2],[34,2],[35,2],
            [11,3],[15,3],[20,3],[21,3],[34,3],[35,3],[0,4],[1,4],[10,4],
            [16,4],[20,4],[21,4],[0,5],[1,5],[10,5],[14,5],[16,5],[17,5],
            [22,5],[24,5],[10,6],[16,6],[24,6],[11,7],[15,7],[12,8],[13,8]
        ];
        pattern.forEach(([x, y]) => {
            const gridY = startY + y;
            const gridX = startX + x;
            if (gridY >= 0 && gridY < rows && gridX >= 0 && gridX < cols) {
                grid[gridY][gridX] = 1;
            }
        });
    }
    
    // Place a single Gosper Gun
    addGosperGun(10, 10);
    
    function countNeighbors(grid, x, y) {
        let count = 0;
        for (let i = -1; i <= 1; i++) {
            for (let j = -1; j <= 1; j++) {
                if (i === 0 && j === 0) continue;
                const row = (y + i + rows) % rows;
                const col = (x + j + cols) % cols;
                count += grid[row][col];
            }
        }
        return count;
    }
    
    function nextGeneration() {
        const newGrid = Array(rows).fill().map(() => Array(cols).fill(0));
        for (let y = 0; y < rows; y++) {
            for (let x = 0; x < cols; x++) {
                const neighbors = countNeighbors(grid, x, y);
                if (grid[y][x] === 1) {
                    newGrid[y][x] = (neighbors === 2 || neighbors === 3) ? 1 : 0;
                } else {
                    newGrid[y][x] = neighbors === 3 ? 1 : 0;
                }
            }
        }
        grid = newGrid;
    }
    
    function drawGame() {
        if (!conwayEnabled) return;
        
        const isDark = document.body.classList.contains('dark-mode');
        const isHacker = document.body.classList.contains('hacker-mode');
        
        nextGeneration();
        
        // Clear canvas with transparency
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        
        // Draw cells with very subtle semi-transparent theme color
        for (let y = 0; y < rows; y++) {
            for (let x = 0; x < cols; x++) {
                if (grid[y][x] === 1) {
                    if (isHacker) {
                        ctx.fillStyle = 'rgba(0, 255, 0, 0.08)';
                    } else if (isDark) {
                        ctx.fillStyle = 'rgba(255, 255, 255, 0.05)';
                    } else {
                        ctx.fillStyle = 'rgba(0, 0, 0, 0.05)';
                    }
                    ctx.fillRect(x * cellSize, y * cellSize, cellSize - 1, cellSize - 1);
                }
            }
        }
    }
    
    conwayGame = setInterval(drawGame, 100);
    drawGame();
}

function stopConway() {
    if (conwayGame) {
        clearInterval(conwayGame);
        conwayGame = null;
    }
    const canvas = document.getElementById('snake-canvas');
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
}

// Handle window resize
window.addEventListener('resize', () => {
    if (conwayEnabled) {
        stopConway();
        startConway();
    }
});

function toggleTheme() {
    const body = document.body;
    const html = document.documentElement;
    const button = document.querySelector('.theme-toggle');
    
    // Check for quintuple click
    const now = Date.now();
    if (!window.lastThemeClick) window.lastThemeClick = [];
    window.lastThemeClick.push(now);
    
    // Keep only last 5 clicks within 1 second
    window.lastThemeClick = window.lastThemeClick.filter(time => now - time < 1000);
    
    // Five clicks detected - toggle hacker mode
    if (window.lastThemeClick.length >= 5) {
        window.lastThemeClick = [];
        
        if (body.classList.contains('hacker-mode')) {
            // Exit hacker mode to dark
            body.classList.remove('hacker-mode');
            body.classList.add('dark-mode');
            html.classList.remove('hacker-mode');
            html.classList.add('dark-mode');
            button.textContent = '[+]';
            localStorage.setItem('theme', 'dark');
        } else {
            // Enter hacker mode
            body.classList.remove('dark-mode');
            body.classList.add('hacker-mode');
            html.classList.remove('dark-mode');
            html.classList.add('hacker-mode');
            button.textContent = '[H]';
            localStorage.setItem('theme', 'hacker');
        }
        return;
    }
    
    // Normal single click - toggle between light and dark only
    if (body.classList.contains('hacker-mode')) {
        // If in hacker mode, go to dark
        body.classList.remove('hacker-mode');
        body.classList.add('dark-mode');
        html.classList.remove('hacker-mode');
        html.classList.add('dark-mode');
        button.textContent = '[+]';
        localStorage.setItem('theme', 'dark');
    } else if (body.classList.contains('dark-mode')) {
        // Dark to light
        body.classList.remove('dark-mode');
        html.classList.remove('dark-mode');
        button.textContent = '[-]';
        localStorage.setItem('theme', 'light');
    } else {
        // Light to dark
        body.classList.add('dark-mode');
        html.classList.add('dark-mode');
        button.textContent = '[+]';
        localStorage.setItem('theme', 'dark');
    }
}

// Load saved theme on page load
window.addEventListener('DOMContentLoaded', function() {
    const savedTheme = localStorage.getItem('theme');
    const button = document.querySelector('.theme-toggle');
    const html = document.documentElement;
    
    // If light mode is saved, switch to light
    if (savedTheme === 'light') {
        document.body.classList.remove('dark-mode');
        document.body.classList.remove('hacker-mode');
        html.classList.remove('dark-mode');
        html.classList.remove('hacker-mode');
    } else if (savedTheme === 'hacker') {
        // Hacker mode
        document.body.classList.remove('dark-mode');
        document.body.classList.add('hacker-mode');
        html.classList.remove('dark-mode');
        html.classList.add('hacker-mode');
        button.textContent = '[H]';
    } else {
        // Default to dark mode (already set in body tag)
        document.body.classList.add('dark-mode');
        html.classList.add('dark-mode');
        button.textContent = '[+]';
        if (!savedTheme) {
            localStorage.setItem('theme', 'dark');
        }
    }
    
    // Load section from URL hash
    const hash = window.location.hash.substring(1);
    if (hash) {
        // Check if it's a direct entry link (blog post or project)
        const entryElement = document.getElementById(hash);
        if (entryElement) {
            // Find which section it belongs to
            const section = entryElement.closest('.section');
            if (section) {
                showSection(section.id);
                // Open the specific entry
                const title = entryElement.querySelector('.collapsible-title');
                if (title) {
                    const container = title.nextElementSibling;
                    const indicator = title.querySelector('.toggle-indicator');
                    
                    // Show the container
                    if (container) {
                        container.classList.add('show');
                        if (indicator) {
                            indicator.textContent = '[-]';
                        }
                        
                        // Process markdown
                        const markdownDivs = container.querySelectorAll('.markdown');
                        markdownDivs.forEach(div => {
                            if (!div.hasAttribute('data-processed')) {
                                const markdownText = div.textContent || div.innerText;
                                if (markdownText && markdownText.trim()) {
                                    div.innerHTML = marked.parse(markdownText);
                                    div.setAttribute('data-processed', 'true');
                                }
                            }
                        });
                    }
                    
                    // Scroll to the entry after a brief delay
                    setTimeout(() => {
                        entryElement.scrollIntoView({ behavior: 'smooth', block: 'start' });
                    }, 100);
                }
            }
        } else if (document.getElementById(hash)) {
            // It's a section link
            showSection(hash);
        }
    }
});

function copyEntryLink(link) {
    const url = window.location.origin + window.location.pathname + link.getAttribute('href');
    navigator.clipboard.writeText(url).then(() => {
        const originalText = link.textContent;
        link.textContent = '✓';
        setTimeout(() => {
            link.textContent = originalText;
        }, 1500);
    }).catch(err => {
        console.error('Failed to copy:', err);
    });
}

function copyCode(button) {
    const code = button.nextElementSibling;
    navigator.clipboard.writeText(code.textContent).then(() => {
        const originalText = button.textContent;
        button.textContent = 'Copied!';
        setTimeout(() => {
            button.textContent = originalText;
        }, 2000);
    }).catch(err => {
        console.error('Failed to copy:', err);
    });
}

function toggleCode(element) {
    // Check if it's a collapsible title (h3) or old-style link
    let container, indicator;
    
    if (element.tagName === 'H3' && element.classList.contains('collapsible-title')) {
        // New style: h3 is clickable
        container = element.nextElementSibling;
        indicator = element.querySelector('.toggle-indicator');
    } else {
        // Old style: link inside element
        container = element.parentElement.nextElementSibling;
        indicator = element;
    }
    
    if (!container) return;
    
    if (container.classList.contains('show')) {
        container.classList.remove('show');
        if (indicator) {
            indicator.textContent = indicator.classList?.contains('toggle-indicator') ? '[+]' : '[show]';
        }
        // Update URL to just the section when closing
        const section = container.closest('.section');
        if (section) {
            window.history.pushState(null, '', '#' + section.id);
        }
    } else {
        container.classList.add('show');
        if (indicator) {
            indicator.textContent = indicator.classList?.contains('toggle-indicator') ? '[-]' : '[hide]';
        }
        
        // Update URL to the specific entry when opening
        const entryContainer = element.closest('[id]');
        if (entryContainer && entryContainer.id && !entryContainer.classList.contains('section')) {
            window.history.pushState(null, '', '#' + entryContainer.id);
        }
        
        // Process markdown if it hasn't been processed yet
        const markdownDivs = container.querySelectorAll('.markdown');
        markdownDivs.forEach(div => {
            if (!div.hasAttribute('data-processed')) {
                // Get the raw text content
                const markdownText = div.textContent || div.innerText;
                if (markdownText && markdownText.trim()) {
                    div.innerHTML = marked.parse(markdownText);
                    div.setAttribute('data-processed', 'true');
                }
            }
        });
    }
}

function showSection(sectionId) {
    const sections = document.querySelectorAll('.section');
    sections.forEach(section => {
        section.style.display = 'none';
    });
    document.getElementById(sectionId).style.display = 'block';
    
    // Update active link
    const links = document.querySelectorAll('nav a[data-section]');
    links.forEach(link => {
        link.classList.remove('active');
        if (link.getAttribute('data-section') === sectionId) {
            link.classList.add('active');
        }
    });
    
    // Update URL hash
    window.location.hash = sectionId;
}


// Process markdown content on page load
window.addEventListener('DOMContentLoaded', function() {
    // Don't process all markdown on load - only process when entries are opened
    // This prevents issues with hidden content
});
