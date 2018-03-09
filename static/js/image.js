(function() {

  $(window).on('drop', drop);
  $(window).on({'dragenter': prevent, 'dragover': prevent, 'dragleave': prevent});

  function prevent(e) {
    e.preventDefault();
  }

  function drop(e) {
    e.preventDefault();

    var files = e.originalEvent.dataTransfer.files;

    // normal file drag & drop
    if (files.length > 0) {
      for ( var i = 0, file; file = files[i]; ++i ) {
        if ( !files[i].type.match(/image.*/) ) {
          alert('Not an image');
          continue;
        }
        $.ajax({
          headers: { 'filename': file.name },
          url: '/admin/image',
          type: 'POST',
          data: file,
          processData: false,
          contentType: file.type,
          success: function(url) {
            var textArea = $('textarea');
            textArea.val(textArea.val() + '\r\n![](' + url + ')\r\n');
            textArea.oninput();
          },
          error: function(res) {
            alert('Image upload failed');
          }
        });
      }
    // drag & drop from another website
    } else if (e.originalEvent.dataTransfer.getData('text/html').match(/src\s*=\s*"(.+?)"/).length > 1) {
      var url = e.originalEvent.dataTransfer.getData('text/html').match(/src\s*=\s*"(.+?)"/)[1];
      var filename = url.substr(url.lastIndexOf('/') + 1);
      var fileType = 'image/' + url.substr(url.lastIndexOf('.') + 1);
      var xhr = new XMLHttpRequest();
      xhr.open("GET", url, true);
      xhr.responseType = "arraybuffer";
      xhr.onload = function(e) {
        var arrayBufferView = new Uint8Array(this.response);
        var blob = new Blob([arrayBufferView], { type: fileType });
        $.ajax({
          headers: { 'filename': filename },
          url: '/admin/image',
          type: 'POST',
          data: blob,
          processData: false,
          contentType: blob.type,
          success: function(url) {
            var textArea = $('textarea');
            textArea.val(textArea.val() + '\r\n![](' + url + ')\r\n');
          },
          error: function(res) {
            alert('Image upload failed');
          }
        });
      };
      xhr.send();
    }

  }

})();