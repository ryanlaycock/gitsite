function populatePage() {
    loadHeaders();
}


function loadHeaders() {
    $.getJSON("/api/header/links", function(resp) {
        let links = "";
        resp.data.forEach(function(link, k) {
            links += "<li><a href='/"+link.path+"'><div class='header-link-div'>"+link.name+"</div></a></li>";
        });
        $("#links").html(links);
    });
}
