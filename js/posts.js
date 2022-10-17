var apple = "apple"
        document.addEventListener("DOMContentLoaded", function (event) {
            var getTemplateAsString = document.getElementById("h1-template").innerHTML
            var compiledContent = Handlebars.compile(getTemplateAsString)
            document.getElementById("h1-template").innerHTML = compiledContent({ h1Template: apple })
        });