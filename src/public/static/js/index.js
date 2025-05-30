// DOM Elements
const difficultySlider = document.getElementById('difficulty');
const difficultyValue = document.getElementById('difficulty-value');
const formatSelect = document.getElementById('format');
const charactersInput = document.getElementById('characters');
const generateBtn = document.getElementById('generate-btn');
const captchaDisplay = document.getElementById('captcha-display');
const statusBadge = document.getElementById('status-badge');
const formatBadge = document.getElementById('format-badge');
const apiResponseCode = document.getElementById('api-response-code');

// Tab functionality
const tabButtons = document.querySelectorAll('.tab-button');
const tabPanels = document.querySelectorAll('.tab-panel');
const codeTabButtons = document.querySelectorAll('.code-tab-button');
const codeExamples = document.querySelectorAll('.code-example-card');
const copyButtons = document.querySelectorAll('.copy-btn');

// Demo state
let isGenerating = false;

// Initialize the application
function init() {
    setupEventListeners();
    updateFormatBadge();
    setupTabs();
    setupCodeTabs();
    setupCopyButtons();
}

// Setup event listeners
function setupEventListeners() {
    // Difficulty slider
    difficultySlider.addEventListener('input', (e) => {
        difficultyValue.textContent = e.target.value;
    });

    // Format select
    formatSelect.addEventListener('change', updateFormatBadge);

    // Generate button
    generateBtn.addEventListener('click', generateCaptcha);

    // Smooth scrolling for anchor links
    document.addEventListener('click', (e) => {
        if (e.target.matches('a[href^="#"]')) {
            e.preventDefault();
            const target = document.querySelector(e.target.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        }
    });
}

// Update format badge
function updateFormatBadge() {
    const format = formatSelect.value.toUpperCase();
    formatBadge.textContent = format;
    updateApiResponse();
}

function scrollToSection(section) {
    document.getElementById(section).scrollIntoView({
      behavior: "smooth"
    });
}

// Generate captcha simulation
async function generateCaptcha() {
    if (isGenerating) return;

    isGenerating = true;
    generateBtn.textContent = 'Generating...';
    generateBtn.disabled = true;
    statusBadge.textContent = 'Generating...';
    statusBadge.className = 'badge badge-secondary';

    // Show loading spinner
    captchaDisplay.innerHTML = `
        <div class="loading-spinner"></div>
    `;

    const response = await fetch(`${window.location.protocol}/captcha.json?height=200&width=500`, { method : "GET" })
    .then(res => res.json())
    .catch(error => {
        console.log(error);
        return { "image" : null, "text" : null, "error" : error };
    })

    // Update display
    captchaDisplay.innerHTML = `
        <img src="${response.image}">
        <!--
        <div class="captcha-placeholder">
            <div class="captcha-sample">
                <img src="${response.image}">
                <span class="captcha-text"></span>
            </div>
            <p class="captcha-label">Generated captcha image</p>
        </div>
        -->
    `;

    // Reset button and status
    isGenerating = false;
    generateBtn.textContent = 'Generate Captcha';
    generateBtn.disabled = false;
    statusBadge.textContent = 'Ready';
    statusBadge.className = 'badge badge-success';

    // Update API response with new data
    //updateApiResponse(captchaText.toLowerCase());
}

// Update API response display
function updateApiResponse(captchaId = 'abc123') {
    const format = formatSelect.value;
    const response = {
        id: `cap_${captchaId}`,
        image_url: `/captcha/${captchaId}.${format}`,
        expires_at: "2024-01-01T12:00:00Z"
    };

    apiResponseCode.textContent = JSON.stringify(response, null, 2);
}

// Setup main tabs functionality
function setupTabs() {
    tabButtons.forEach(button => {
        button.addEventListener('click', () => {
            const tabName = button.getAttribute('data-tab');
            
            // Remove active class from all buttons and panels
            tabButtons.forEach(btn => btn.classList.remove('active'));
            tabPanels.forEach(panel => panel.classList.remove('active'));
            
            // Add active class to clicked button and corresponding panel
            button.classList.add('active');
            document.getElementById(`${tabName}-tab`).classList.add('active');
        });
    });
}

// Setup code example tabs functionality
function setupCodeTabs() {
    codeTabButtons.forEach(button => {
        button.addEventListener('click', () => {
            const tabName = button.getAttribute('data-code-tab');
            
            // Remove active class from all buttons and examples
            codeTabButtons.forEach(btn => btn.classList.remove('active'));
            codeExamples.forEach(example => example.classList.add('hidden'));
            
            // Add active class to clicked button and show corresponding example
            button.classList.add('active');
            document.getElementById(`${tabName}-example`).classList.remove('hidden');
        });
    });
}

// Setup copy to clipboard functionality
function setupCopyButtons() {
    copyButtons.forEach(button => {
        button.addEventListener('click', async () => {
            const codeType = button.getAttribute('data-copy');
            const codeExample = document.querySelector(`#${codeType}-example .code-block code`);
            
            if (codeExample) {
                try {
                    await navigator.clipboard.writeText(codeExample.textContent);
                    
                    // Show feedback
                    const originalText = button.textContent;
                    button.textContent = 'Copied!';
                    button.style.background = 'var(--color-success)';
                    button.style.color = 'var(--color-white)';
                    
                    // Reset after 2 seconds
                    setTimeout(() => {
                        button.textContent = originalText;
                        button.style.background = '';
                        button.style.color = '';
                    }, 2000);
                } catch (err) {
                    console.error('Failed to copy text: ', err);
                    
                    // Fallback: select text
                    const selection = window.getSelection();
                    const range = document.createRange();
                    range.selectNodeContents(codeExample);
                    selection.removeAllRanges();
                    selection.addRange(range);
                }
            }
        });
    });
}

// Add scroll animations
function setupScrollAnimations() {
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.style.opacity = '1';
                entry.target.style.transform = 'translateY(0)';
            }
        });
    }, observerOptions);

    // Observe elements for animation
    const animatedElements = document.querySelectorAll('.card, .feature-card, .endpoint-card');
    animatedElements.forEach(el => {
        el.style.opacity = '0';
        el.style.transform = 'translateY(20px)';
        el.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
        observer.observe(el);
    });
}

// Add hover effects for cards
function setupHoverEffects() {
    const cards = document.querySelectorAll('.card, .feature-card');
    
    cards.forEach(card => {
        card.addEventListener('mouseenter', () => {
            card.style.transform = 'translateY(-4px)';
        });
        
        card.addEventListener('mouseleave', () => {
            card.style.transform = 'translateY(0)';
        });
    });
}

// Initialize everything when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    init();
    setupScrollAnimations();
    setupHoverEffects();
});

// Add some easter eggs and interactions
document.addEventListener('keydown', (e) => {
    // Press 'g' to generate captcha
    if (e.key === 'g' && !isGenerating && document.activeElement.tagName !== 'INPUT') {
        e.preventDefault();
        generateCaptcha();
    }
    
    // Press 'Escape' to close any modal-like content (future enhancement)
    if (e.key === 'Escape') {
        // Could be used for modals or overlays in the future
    }
});

// Add loading states for better UX
function addLoadingState(element, text = 'Loading...') {
    const originalContent = element.innerHTML;
    element.innerHTML = `<span class="loading-spinner"></span> ${text}`;
    element.disabled = true;
    
    return () => {
        element.innerHTML = originalContent;
        element.disabled = false;
    };
}

// Utility function for debouncing
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

// Add resize listener for responsive behavior
window.addEventListener('resize', debounce(() => {
    // Could add responsive adjustments here if needed
    console.log('Window resized');
}, 250));

// Export functions for potential external use
window.CaptchaAPI = {
    generateCaptcha,
    updateFormatBadge,
    setupTabs,
    setupCodeTabs
};