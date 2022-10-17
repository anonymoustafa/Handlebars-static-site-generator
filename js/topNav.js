document.addEventListener("DOMContentLoaded", function (event) {
            var getTemplateAsString = document.getElementById("topNavTemplate").innerHTML
            var compiledContent = Handlebars.compile(getTemplateAsString)
            document.getElementById("topNavTemplate").innerHTML =
            compiledContent({ topNavTemplate: topNavLinksAndContents })
        });