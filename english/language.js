(function () {
  var url = window.location.href;
  var host = window.location.host;
  var home_url = window.location.protocol + '//' + window.location.host;

  var search = {
    en: "/en/",
    zh_CN: "/zh-CN/"
  }

  var replaceWith = {
    en: "/zh-CN/",
    zh_CN: "/en/"
  }

  console.log("hheheeh");

  var link = "";
  var word = "";
  var home = "Home";

  var edit_url = "https://github.com/rust-lang-cn"
  var github_id = document.getElementById("git-repository-button");
  if (github_id != null && github_id.parentNode.href != undefined) {
    edit_url = github_id.parentNode.href;
    var repo_name = edit_url.split('/').pop();
    var url_name = repo_name.slice(0, -3);
    html_file = url.split(url_name).pop();
    
    if (html_file == '/') {
      edit_url += '/tree/master/src'
    } else {
      md_len = html_file.indexOf('.html');
      if (md_len > -1) {
        md_file = html_file.substring(0, md_len) + '.md';
        edit_url += '/blob/master/src' + md_file;
      }
    }
  }

  var edit_node = '<a href="' + edit_url + '" title="改进本页翻译" aria-label="改进本页翻译"><i id="go-back-homepage" class="fa fa-pencil-square-o"></i></a>';

  if (url.indexOf(search.en) != -1 && url.indexOf(search.en) === (url.indexOf(host) + host.length)) {
    link = url.replace(search.en, replaceWith.en);
    word = "简体中文";
    edit_node = "";
  } else if (url.indexOf(search.zh_CN) != -1 && url.indexOf(search.zh_CN) === (url.indexOf(host) + host.length)) {
    link = url.replace(search.zh_CN, replaceWith.zh_CN);
    word = "English";
	  home = "首页";
  }

  var home_node = '';
  if (window.location.protocol == 'http:' || window.location.protocol == 'https:') {
    home_node = '<a href="' + home_url + '" title="' + home + '" aria-label="' + home + '"><i id="go-back-homepage" class="fa fa-home"></i></a>';
  }
  var lang_node = '';
  if (link != '') {
    lang_node = '<a href="' + link + '"><i id="change-language" class="fa fa-language"> ' + word + '</i></a>';
  }
  var insertNode = document.getElementsByClassName('right-buttons');
  if (insertNode.length > 0) {
    var html = insertNode[0].innerHTML;
    insertNode[0].innerHTML = home_node + html + edit_node + lang_node;
  }
})()  
