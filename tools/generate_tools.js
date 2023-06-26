// This tool create a json file based on the "Dependency" struct 

const fs = require("fs")
const path = require("path")

/**
 * From [awesome-typst](https://github.com/qjcg/awesome-typst/)
 * Formated and then use to make the file "list_projects"
 */
const listLinks = 
`https://gitea.everydayimshuflin.com/greg/typst-lepizig-glossing
https://github.com/1bitbool/SimplePaper
https://github.com/8LWXpg/typst-ansi_render
https://github.com/AntoniosBarotsis/typst-assignment-template
https://github.com/GeorgeHoneywood/alta-typst
https://github.com/Harkunwar/attractive-typst-resume
https://github.com/LLBlumire/writable-gm-screen-inserts
https://github.com/LaurenzV/simplecv
https://github.com/Leedehai/typst-physics
https://github.com/OriginCode/typst-homework-template
https://github.com/Pegacraft/typst-plotting
https://github.com/PgBiel/typst-diagbox
https://github.com/PgBiel/typst-strfmt
https://github.com/PgBiel/typst-tablex
https://github.com/RolfBremer/typst-glossary
https://github.com/RolfBremer/typst-index
https://github.com/SeniorMars/typst-raytracer
https://github.com/andreasKroepelin/typst-slides
https://github.com/asibahi/simple-poem-typst
https://github.com/astrale-sharp/typst-assignement-template.git
https://github.com/aurghya-0/Project-Report-Typst
https://github.com/bamboovir/typst-resume-template
https://github.com/bsp0109/ieee-typst-template
https://github.com/daxartio/cv
https://github.com/dvdvgt/typst-letter
https://github.com/eigenein/typst-templates
https://github.com/elegaanz/vercanard
https://github.com/erictapen/typst-invoice
https://github.com/fenjalien/circuitypst
https://github.com/gRox167/typst-assignment-template.git
https://github.com/giovanniberti/moderncv.typst
https://github.com/gvariable/billryan-typst
https://github.com/haxibami/typst-template
https://github.com/hexWars/resume
https://github.com/ice-kylin/typst-cv-miku
https://github.com/imatpot/typst-ipa
https://github.com/johannes-wolf/typst-canvas
https://github.com/johannes-wolf/typst-plot
https://github.com/johanvx/typst-undergradmath
https://github.com/jomaway/typst-bytefield
https://github.com/jomaway/typst-teacher-templates
https://github.com/jxpeng98/Typst-CV-Resume
https://github.com/kaarmu/typst-palettes
https://github.com/ljgago/typst-chords
https://github.com/lkoehl/typst-boxes
https://github.com/ludwig-austermann/typst-timetable
https://github.com/lvignoli/diapo
https://github.com/mintyfrankie/awesomeCV-Typst
https://github.com/onefact/datathinking.org-report-template
https://github.com/pascal-huber/typst-letter-template
https://github.com/platformer/typst-algorithms
https://github.com/pncnmnp/typst-poster
https://github.com/qjcg/typstry
https://github.com/rinmyo/ruby-typ
https://github.com/saadulkh/typst-notes
https://github.com/sahasatvik/typst-theorems
https://github.com/skyzh/typst-cv-template
https://github.com/tbug/notes.typ
https://github.com/titaneric/typst-mixed-resume
https://github.com/wusyong/resume.typ
https://github.com/wychwitch/tyspt-mla9-template
https://github.com/zagoli/simple-typst-thesis
https://gitlab.com/giacomogallina/typst-cd
https://gitlab.com/jim.hefferon/undergradmath`
.split("\n").map(a => a.split(".git")[0]);

/**
 * Based on the struct "Dependency"
 */
const listProjects = listLinks.map(a => {
    const l = a.split("/");
    return {
        name: l[4],
        version: "latest",
        link: a + ".git",
        author: {
            name: l[3],
            email: null,
            website: null,
        },
        authors: null,
        main: "./main.typ",
        auto: true,
    }
})

fs.writeFileSync(path.join(__dirname, "../list_projects.json"), JSON.stringify(listProjects, null, 2))
