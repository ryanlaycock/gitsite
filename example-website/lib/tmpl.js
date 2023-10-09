function populatePage() {
    loadContent();
    loadHeaders();
}

function loadContent() {
    var pathname = "/content" + window.location.pathname;
    
    $.getJSON(pathname, function(resp) {
        $("#content").html(resp.data);
    });
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
