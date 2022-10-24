var footer = `
<p>Author: Mostafa Ramezani<br>
<a href="mailto:anonymoustafa@outlook.com">anonymoustafa[at symbol]outlook.com</a></p>
`

        document.addEventListener("DOMContentLoaded", function (event) {
            var getTemplateAsString = document.getElementById("footerTemplate").innerHTML
            var compiledContent = Handlebars.compile(getTemplateAsString)
            document.getElementById("footerTemplate").innerHTML = compiledContent({ footerTemplate: footer })
        });