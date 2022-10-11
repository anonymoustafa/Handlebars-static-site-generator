document.addEventListener("DOMContentLoaded", function (event) {
    var getTemplateAsString = document.getElementById("h1-template").textContent
    var compiledContent = Handlebars.compile(getTemplateAsString)
    document.getElementById("h1-template").innerHTML = compiledContent({template:"apple"})
});

