var topNavLinksAndContents = `
<ul>
<li class="topNavBar"><a href="/about">About</a></li>
<li class="topNavBar"><a href="/about">About</a></li>
<li class="topNavBar"><a href="/contact" >Contact</a></li>
<li class="topNavBar"><a href="/press"> so</a></li>
<li>
    <div class="dropdown">
        <button onclick="myFunction()" class="dropbtn">Posts</button>
        <div id="myDropdown" class="dropdown-content">
            <a href="#ce">Civil & Environmental Eng</a>
            <a href="#wjs">Web and JavaScript</a>
            <a href="#ls">Linux and system development</a>
        </div>
    </div>
</li>
<li class="searchBox"><input type="text" placeholder="Search..."></li>
</ul>
`

document.addEventListener("DOMContentLoaded", function (event) {
            var getTemplateAsString = document.getElementById("topNavTemplate").innerHTML
            var compiledContent = Handlebars.compile(getTemplateAsString)
            document.getElementById("topNavTemplate").innerHTML =
            compiledContent({ topNavTemplate: topNavLinksAndContents })
        });