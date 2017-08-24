// Taken from https://github.com/dbushenko/purescript-cookies/blob/master/src/Web/Cookies.js
"use strict";
// module Cookie

exports._getCookie =
  function(name) {
    return function() {
      var matches = document.cookie.match(new RegExp(
        "(?:^|; )" + name.replace(/([\.$?*|{}\(\)\[\]\\\/\+^])/g, '\\$1') + "=([^;]*)"
      ));
      var data = matches ? decodeURIComponent(matches[1]) : undefined;

      if (data == undefined || data == null) {
        return [];
      } else {
        return [data];
      }
    };
  }
