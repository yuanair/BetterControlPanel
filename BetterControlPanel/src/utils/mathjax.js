window.MathJax = {
    loader: {
        load: ['[tex]/mhchem'],
    },
    chtml: {mtextInheritFont: true},   // 确保文本继承字体
    tex: {
        packages: {'[+]': ['mhchem']},
        inlineMath: [['$', '$'], ['\\(', '\\)']],
        displayMath: [['$$', '$$'], ['\$$', '\$$']],
    },
    options: {
        enableMenu: false,      // 彻底禁用菜单系统
        menuOptions: {
            settings: {
                zoom: 'None',       // 禁用缩放
            }
        }
    },
};