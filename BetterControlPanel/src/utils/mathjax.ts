window.MathJax = {
    tex: {
        inlineMath: [["$", "$"], ["\\(", "\\)"]], //行内公式选择符
        displayMath: [["$$", "$$"], ["\\[", "\\]"]], //段内公式选择符
        tagSide: 'left',
        macros: {
            RR: '{\\bf R}',
            bold: ['{\\bf #1}', 1]
        }
    },
    startup: {
        pageReady: () => {
            return MathJax.startup.defaultPageReady();
        }
    },
    options: {
        skipHtmlTags: [
            'script', 'noscript', 'style', 'textarea', 'pre',
            'code', 'annotation', 'annotation-xml'
        ],
    }
};
import {nextTick} from 'vue'

import {onMounted} from "vue";

onMounted(() => {
    MathJax.typesetPromise()
});

function useRenderMath() {
    nextTick(() => {
        MathJax.texReset();
        MathJax.typesetClear();
        MathJax.typesetPromise();
    })
}

export default useRenderMath