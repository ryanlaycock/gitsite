loadContent();

function loadContent() {
    var pathname = "/content" + window.location.pathname;
    $(function(){
        $.getJSON(pathname, function(jd) {
            $("#content").html(jd.data);
         });
    });
}
