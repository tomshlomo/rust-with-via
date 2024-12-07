// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Welcome to Rust with Via</a></li><li class="chapter-item expanded affix "><a href="schedule.html">Schedule</a></li><li class="chapter-item expanded affix "><a href="tips.html">Miscellaneous Tips</a></li><li class="chapter-item expanded affix "><li class="part-title">Creating Python Extension Modules</li><li class="chapter-item expanded "><a href="pyo3/index.html"><strong aria-hidden="true">1.</strong> Welcome</a></li><li class="chapter-item expanded "><a href="pyo3/maturin.html"><strong aria-hidden="true">2.</strong> Setup</a></li><li class="chapter-item expanded "><a href="pyo3/functions.html"><strong aria-hidden="true">3.</strong> Functions</a></li><li class="chapter-item expanded "><a href="pyo3/classes.html"><strong aria-hidden="true">4.</strong> Classes</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="pyo3/methods.html"><strong aria-hidden="true">4.1.</strong> Methods</a></li></ol></li><li class="chapter-item expanded "><a href="pyo3/errors.html"><strong aria-hidden="true">5.</strong> Error Handling</a></li><li class="chapter-item expanded affix "><li class="part-title">Exercises</li><li class="chapter-item expanded "><a href="inventory/index.html"><strong aria-hidden="true">6.</strong> Inventory</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="inventory/solution.html"><strong aria-hidden="true">6.1.</strong> Solution</a></li><li class="chapter-item expanded "><a href="inventory/instructor.html"><strong aria-hidden="true">6.2.</strong> Instructor Notes</a></li></ol></li><li class="chapter-item expanded "><a href="non-empty-vec/index.html"><strong aria-hidden="true">7.</strong> Non Empty Vector</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="non-empty-vec/solution.html"><strong aria-hidden="true">7.1.</strong> Solution</a></li><li class="chapter-item expanded "><a href="non-empty-vec/instructor.html"><strong aria-hidden="true">7.2.</strong> Instructor Notes</a></li></ol></li><li class="chapter-item expanded "><a href="lifetimes/index.html"><strong aria-hidden="true">8.</strong> Lifetimes</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="lifetimes/solution.html"><strong aria-hidden="true">8.1.</strong> Solution</a></li><li class="chapter-item expanded "><a href="lifetimes/instructor.html"><strong aria-hidden="true">8.2.</strong> Instructor Notes</a></li></ol></li><li class="chapter-item expanded "><a href="mini-gtfs/index.html"><strong aria-hidden="true">9.</strong> Mini GTFS</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="mini-gtfs/solution.html"><strong aria-hidden="true">9.1.</strong> Solution</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="mini-gtfs/simple_solution.html"><strong aria-hidden="true">9.1.1.</strong> Simple Solution</a></li><li class="chapter-item expanded "><a href="mini-gtfs/faster_solution.html"><strong aria-hidden="true">9.1.2.</strong> Faster Solution</a></li><li class="chapter-item expanded "><a href="mini-gtfs/instructor.html"><strong aria-hidden="true">9.1.3.</strong> Instructor Notes</a></li></ol></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
