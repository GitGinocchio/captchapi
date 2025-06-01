function scrollToSection(section) {
    document.getElementById(section).scrollIntoView({
      behavior: "smooth"
    });
}

function scrollToTab(section, tab) {
    scrollToSection(section)

    if (tab == "examples") {
        document.getElementById("examples-btn").click();
    }
    else if (tab == "endpoints") {
        document.getElementById("endpoints-btn").click();
    }
}

document.addEventListener("DOMContentLoaded", (event) => {
let isGenerating = false;
let scrolling = false;
let touchStartY = 0;

let captchaText = null;

let currentStep = 1;
const totalSteps = document.querySelectorAll('.step').length;

const generateBtn = document.getElementById("generate-btn");
const verifyBtn = document.getElementById("verify-btn");
const captchaDisplay = document.getElementById('captcha-display');
const paginationDots = document.getElementById('pagination-dots');
const stepContainer = document.getElementById('step-container');

const tabButtons = document.querySelectorAll('.tab-button');
const tabPanels = document.querySelectorAll('.tab-panel');
const codeTabButtons = document.querySelectorAll('.code-tab-button');
const codeExamples = document.querySelectorAll('.code-example-card');
const copyButtons = document.querySelectorAll('.copy-btn');

// Sliders
const letters_slider = document.getElementById('letters-slider');
const lines_slider = document.getElementById('lines-slider');
const circles_slider = document.getElementById('circles-slider');
const paths_slider = document.getElementById("paths-slider");
const blur_slider = document.getElementById('blur-slider');
const opacity_slider = document.getElementById('opacity-slider');
const fsize_slider = document.getElementById('fsize-slider');
const rotation_slider = document.getElementById('rotation-slider');
const lscalex_slider = document.getElementById('lscalex-slider');
const lscaley_slider = document.getElementById('lscaley-slider');
const lwidth_slider = document.getElementById('lwidth-slider');
const lopacity_slider = document.getElementById('lopacity-slider');
const cradius_slider = document.getElementById('cradius-slider');
const copacity_slider = document.getElementById('copacity-slider');
const pstroke_slider = document.getElementById('pstroke-slider');
const plength_slider = document.getElementById('plength-slider');
const popacity_slider = document.getElementById('popacity-slider');

const generateCaptchaDebounced = debounce(generateCaptcha, 750);
generateBtn.addEventListener("click", generateCaptchaDebounced);

function setupSliders() {
    noUiSlider.create(letters_slider, {
        start: [4, 6],
        connect: true,
        range: { 'min': 1, 'max': 6 },
        step: 1,
        tooltips: true,
        format: {
            to: value => Math.round(value),
            from: value => Number(value)
        }
    });

    noUiSlider.create(lines_slider, {
        start: [3, 5],
        connect: true,
        range: { 'min': 0, 'max': 30 },
        step: 1,
        tooltips: true,
        format: {
            to: value => Math.round(value),
            from: value => Number(value)
        }
    });

    noUiSlider.create(circles_slider, {
        start: [5, 10],
        connect: true,
        range: { 'min': 0, 'max': 250 },
        step: 1,
        tooltips: true,
        format: {
            to: value => Math.round(value),
            from: value => Number(value)
        }
    });

    noUiSlider.create(paths_slider, {
        start: [5, 10],
        connect: true,
        range: { 'min': 0, 'max': 250 },
        step: 1,
        tooltips: true,
        format: {
            to: value => Math.round(value),
            from: value => Number(value)
        }
    });

    noUiSlider.create(blur_slider, {
        start: [0.0, 0.5],
        connect: true,
        range: { 'min': 0, 'max': 1 },
        step: 0.025,
        tooltips: true,
        format: {
            to: value => value,
            from: value => Number(value)
        }
    });

    noUiSlider.create(opacity_slider, {
        start: [0.3, 1.0],
        connect: true,
        range: { 'min': 0, 'max': 1 },
        step: 0.025,
        tooltips: true,
        format: {
            to: value => value,
            from: value => Number(value)
        }
    });

    noUiSlider.create(fsize_slider, {
        start: [18.0, 30.0],
        connect: true,
        range: { 'min': 5.0, 'max': 50.0 },
        step: 0.5,
        tooltips: true,
        format: {
            to: value => parseFloat(value.toFixed(1)),
            from: value => Number(value)
        }
    });

    noUiSlider.create(rotation_slider, {
        start: [-15.0, 15.0],
        connect: true,
        range: { 'min': -360.0, 'max': 360.0 },
        step: 1.0,
        tooltips: true,
        format: {
            to: value => Math.round(value),
            from: value => Number(value)
        }
    });

    noUiSlider.create(lscalex_slider, {
        start: [0.8, 1.2],
        connect: true,
        range: { 'min': 0.1, 'max': 3.0 },
        step: 0.1,
        tooltips: true,
        format: {
            to: value => parseFloat(value.toFixed(1)),
            from: value => Number(value)
        }
    });

    noUiSlider.create(lscaley_slider, {
        start: [0.8, 1.2],
        connect: true,
        range: { 'min': 0.1, 'max': 3.0 },
        step: 0.1,
        tooltips: true,
        format: {
            to: value => parseFloat(value.toFixed(1)),
            from: value => Number(value)
        }
    });

    noUiSlider.create(lwidth_slider, {
        start: [1.0, 3.0],
        connect: true,
        range: { 'min': 0.1, 'max': 10.0 },
        step: 0.1,
        tooltips: true,
        format: {
            to: value => parseFloat(value.toFixed(1)),
            from: value => Number(value)
        }
    });

    noUiSlider.create(lopacity_slider, {
        start: [0.1, 0.6],
        connect: true,
        range: { 'min': 0.1, 'max': 1.0 },
        step: 0.1,
        tooltips: true,
        format: {
            to: value => parseFloat(value.toFixed(1)),
            from: value => Number(value)
        }
    });

    noUiSlider.create(cradius_slider, {
        start: [1.0, 3.0],
        connect: true,
        range: { 'min': 0.1, 'max': 10.0 },
        step: 0.1,
        tooltips: true,
        format: {
            to: value => parseFloat(value.toFixed(1)),
            from: value => Number(value)
        }
    });

    noUiSlider.create(copacity_slider, {
        start: [0.1, 0.8],
        connect: true,
        range: { 'min': 0.1, 'max': 1.0 },
        step: 0.1,
        tooltips: true,
        format: {
            to: value => parseFloat(value.toFixed(1)),
            from: value => Number(value)
        }
    });

    noUiSlider.create(pstroke_slider, {
        start: [1.0, 3.0],
        connect: true,
        range: { 'min': 0.1, 'max': 10.0 },
        step: 0.1,
        tooltips: true,
        format: {
            to: value => parseFloat(value.toFixed(1)),
            from: value => Number(value)
        }
    });

    noUiSlider.create(plength_slider, {
        start: [10.0, 60.0],
        connect: true,
        range: { 'min': 0.1, 'max': 100.0 },
        step: 0.1,
        tooltips: true,
        format: {
            to: value => parseFloat(value.toFixed(1)),
            from: value => Number(value)
        }
    });

    noUiSlider.create(popacity_slider, {
        start: [0.1, 0.6],
        connect: true,
        range: { 'min': 0.1, 'max': 1.0 },
        step: 0.1,
        tooltips: true,
        format: {
            to: value => parseFloat(value.toFixed(1)),
            from: value => Number(value)
        }
    });

    letters_slider.noUiSlider.on('update', generateCaptchaDebounced);
    lines_slider.noUiSlider.on('update', generateCaptchaDebounced);
    circles_slider.noUiSlider.on('update', generateCaptchaDebounced);
    paths_slider.noUiSlider.on('update', generateCaptchaDebounced);
    blur_slider.noUiSlider.on('update', generateCaptchaDebounced);
    opacity_slider.noUiSlider.on('update', generateCaptchaDebounced);
    fsize_slider.noUiSlider.on('update', generateCaptchaDebounced);
    rotation_slider.noUiSlider.on('update', generateCaptchaDebounced);
    lscalex_slider.noUiSlider.on('update', generateCaptchaDebounced);
    lscaley_slider.noUiSlider.on('update', generateCaptchaDebounced);
    lwidth_slider.noUiSlider.on('update', generateCaptchaDebounced);
    lopacity_slider.noUiSlider.on('update', generateCaptchaDebounced);
    cradius_slider.noUiSlider.on('update', generateCaptchaDebounced);
    copacity_slider.noUiSlider.on('update', generateCaptchaDebounced);
    pstroke_slider.noUiSlider.on('update', generateCaptchaDebounced);
    plength_slider.noUiSlider.on('update', generateCaptchaDebounced);
    popacity_slider.noUiSlider.on('update', generateCaptchaDebounced);
}

function setupPaginationDots() {
    for (let i = 0; i < totalSteps; i++) {
        const dot = document.createElement('div');
        dot.classList.add('dot');
        if (i == 0) {
            dot.classList.add('active');
        }
        paginationDots.appendChild(dot);
    }
}

function setupHrefs() {
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

function showStep(step) {
    const steps = document.querySelectorAll('.step');
    steps.forEach((el) => {
        el.classList.remove('active');
        if (parseInt(el.dataset.step) === step) {
            el.classList.add('active');
        }
    });

    const dots = document.querySelectorAll('.dot');
    dots.forEach((dot, index) => {
        dot.classList.toggle('active', index === step - 1);
    });
}

stepContainer.addEventListener('touchstart', (e) => {
    e.preventDefault();
    if (e.target.closest('.noUi-target') != null) return;
    touchStartY = e.touches[0].clientY;
}, { passive: false });

stepContainer.addEventListener('touchend', (e) => {
    e.preventDefault();
    if (scrolling || e.target.closest('.noUi-target') != null) return;
    const touchEndY = e.changedTouches[0].clientY;
    const deltaY = touchStartY - touchEndY;

    // Soglia minima per evitare attivazioni accidentali
    const swipeThreshold = 30;

    if (Math.abs(deltaY) > swipeThreshold) {
        scrolling = true;

        if (deltaY > 0 && currentStep < totalSteps) {
            currentStep++; // swipe up
        } else if (deltaY < 0 && currentStep > 1) {
            currentStep--; // swipe down
        }

        showStep(currentStep);

        setTimeout(() => {
            scrolling = false;
        }, 600);
    }
}, { passive: false });

stepContainer.addEventListener('wheel', (e) => {
    e.preventDefault();
    if (scrolling) return;
    scrolling = true;

    if (e.deltaY > 0 && currentStep < totalSteps) {
        currentStep++;
    } else if (e.deltaY < 0 && currentStep > 1) {
        currentStep--;
    }

    showStep(currentStep);

    setTimeout(() => {
        scrolling = false;
    }, 600); // debounce tempo tra scroll
}, { passive: false });

async function generateCaptcha() {
    if (isGenerating) return;

    isGenerating = true;
    captchaText = null;
    generateBtn.textContent = 'Generating...';
    generateBtn.disabled = true;

    // Show loading spinner
    captchaDisplay.innerHTML = `
        <div class="captcha-placeholder">
            <div class="loading-spinner"></div>
        </div>
    `;

    const [lemin, lemax] = letters_slider.noUiSlider.get().map(Number);
    const [limin, limax] = lines_slider.noUiSlider.get().map(Number);
    const [cmin, cmax] = circles_slider.noUiSlider.get().map(Number);
    const [pmin, pmax] = paths_slider.noUiSlider.get().map(Number);
    const [bmin, bmax] = blur_slider.noUiSlider.get().map(Number);
    const [omin, omax] = opacity_slider.noUiSlider.get().map(Number);
    const [fsmin, fsmax] = fsize_slider.noUiSlider.get().map(Number);
    const [rmin, rmax] = rotation_slider.noUiSlider.get().map(Number);
    const [lsxmin, lsxmax] = lscalex_slider.noUiSlider.get().map(Number);
    const [lsymin, lsymax] = lscaley_slider.noUiSlider.get().map(Number);
    const [lwmin, lwmax] = lwidth_slider.noUiSlider.get().map(Number);
    const [lomin, lomax] = lopacity_slider.noUiSlider.get().map(Number);
    const [crmin, crmax] = cradius_slider.noUiSlider.get().map(Number);
    const [comin, comax] = copacity_slider.noUiSlider.get().map(Number);
    const [psmin, psmax] = pstroke_slider.noUiSlider.get().map(Number);
    const [plmin, plmax] = plength_slider.noUiSlider.get().map(Number);
    const [pomin, pomax] = popacity_slider.noUiSlider.get().map(Number);

    const response = await fetch(`${window.location.protocol}/captcha.json
        ?width=500
        &height=100
        &nletters=${lemin},${lemax}
        &nlines=${limin},${limax}
        &ncircles=${cmin},${cmax}
        &npaths=${pmin},${pmax}
        &blur=${bmin},${bmax}
        &opacity=${omin},${omax}
        &fsize=${fsmin},${fsmax}
        &rotation=${rmin},${rmax}
        &lscalex=${lsxmin},${lsxmax}
        &lscaley=${lsymin},${lsymax}
        &lwidth=${lwmin},${lwmax}
        &lopacity=${lomin},${lomax}
        &cradius=${crmin},${crmax}
        &copacity=${comin},${comax}
        &pstroke=${psmin},${psmax}
        &plength=${plmin},${plmax}
        &popacity=${pomin},${pomax}`
        .replaceAll(" ","").replaceAll("\n",""), 
        { method : "GET" }
    )
    .then(res => res.json())
    .catch(error => {
        console.log(error);
        return { "image" : null, "text" : null, "error" : error };
    })

    if (response.image == null) {
        return;
    }

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
    captchaText = response.text;
    generateBtn.textContent = 'Generate Captcha';
    generateBtn.disabled = false;
}

function debounce(func, delay) {
    let timeoutId;
    return function(...args) {
        clearTimeout(timeoutId);
        timeoutId = setTimeout(() => func.apply(this, args), delay);
    };
}

function init() {
    setupTabs();
    setupCodeTabs();
    setupHrefs();
    setupCopyButtons();
    setupSliders();
    setupScrollAnimations();
    showStep(currentStep);
    setupPaginationDots();
}

init();
});