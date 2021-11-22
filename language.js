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

  var link = "";
  var word = "";
  var home = "Home";
  var lang = "zh-CN";
  var changeLang = "切换到英语";

  if (url.indexOf(search.en) != -1 && url.indexOf(search.en) === (url.indexOf(host) + host.length)) {
    link = url.replace(search.en, replaceWith.en);
    word = "简体中文";
    lang = "en";
    changeLang = "Switch to Chinese"
  } else if (url.indexOf(search.zh_CN) != -1 && url.indexOf(search.zh_CN) === (url.indexOf(host) + host.length)) {
    link = url.replace(search.zh_CN, replaceWith.zh_CN);
    word = "English";
	  home = "首页";
  }

  var edit_id = document.getElementById("git-edit-button");
  if (edit_id != null && edit_id.parentNode != null) {
    edit_id.parentNode.target = "_blank";
    if (lang != "en") {
      edit_id.parentNode.title = "报告错误或改进本页翻译";
    }
  }

  var home_node = '';
  if (window.location.protocol == 'http:' || window.location.protocol == 'https:') {
    home_node = '<a href="' + home_url + '" title="' + home + '" aria-label="' + home + '"><i id="go-back-homepage" class="fa fa-home"></i></a>';
  }
  var lang_node = '';
  if (link != '') {
    lang_node = '<a href="' + link + '" title="' + changeLang + '" aria-label="' + changeLang + '"><i id="change-language-button" class="fa fa-language"> ' + word + '</i></a>';
  }
  var insertNode = document.getElementsByClassName('right-buttons');
  if (insertNode.length > 0) {
    var html = insertNode[0].innerHTML;
    insertNode[0].innerHTML = home_node + html + lang_node;
  }
})()  
