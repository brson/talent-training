// -*- mode: Javascript; indent-tabs-mode: nil; -*-

"use strict";

import * as common from "./common.js";

export function init(config) {
	console.log("lesson slides mode");

    let reveal_mod = "node_modules/reveal.js/js/reveal";
    let marked_mod = "node_modules/reveal.js/plugin/markdown/marked";

    define.amd = false;
    require([reveal_mod], function(_reveal) {

        console.log("holy reveal!");

        let reveal = window.Reveal;

        let mdUrl = common.mdUrlFromUrl(config.url);
        console.log(`md url: ${mdUrl}`);

        // Create the elements needed to run reveal.
        // For simplicity the reveal div is added directly as
        // the first child of the body element, with the nav
        // element positioned absolutely on top of it.

        let revealDiv = document.createElement("div");
        revealDiv.className = "reveal";
        let slidesDiv = document.createElement("div");
        slidesDiv.className = "slides";
        revealDiv.appendChild(slidesDiv);

        let mdSection = document.createElement("section");
        mdSection.setAttribute("data-markdown", mdUrl);
        mdSection.setAttribute("data-charset", "utf-8");
        mdSection.setAttribute("data-separator", "^---")
        mdSection.setAttribute("data-separator-notes", "^Text:")
        slidesDiv.appendChild(mdSection);

        let body = document.querySelector("body");
        body.insertBefore(revealDiv, body.firstChild);

        let revealCfg = {
            history: true,
            controls: true,
            progress: true,
            center: true,
            dependencies: [
                { src: 'node_modules/reveal.js/plugin/markdown/marked.js' },
                { src: 'node_modules/reveal.js/plugin/markdown/markdown.js', callback: function() { define.amd = true; } },
                { src: 'node_modules/reveal.js/plugin/notes/notes.js', async: true }
                //{ src: 'node_modules/reveal.js/plugin/highlight/highlight.js', async: true, callback: function() { fetchAllCode(); hljs.initHighlightingOnLoad(); addButtons(); } },
            ]
        };

        reveal.initialize(revealCfg);
    });
}

