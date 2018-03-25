(function() {

  function Editor(input, preview) {
    this.update = function () {
      var reader = new commonmark.Parser();
      var writer = new commonmark.HtmlRenderer();
      var parsed = reader.parse(input.value);
      preview.innerHTML = writer.render(parsed);
    };
    input.editor = this;
    this.update();
  }
  var $ = function (id) { return document.getElementById(id); };
  new Editor($("markdown"), $("preview"));

})();