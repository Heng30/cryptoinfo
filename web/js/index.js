const normalColor = 'lightgray';
const checkedColor = 'steelblue';
var checkedBtn = null;
var iconBtns = ['home-btn', 'protocol-btn'];
for (var i = 0; i < iconBtns.length; i++) {
  var btn = document.getElementsByClassName(iconBtns[i])[0];
  btn.onclick = function () {
    if (checkedBtn) {
      checkedBtn.style.background = normalColor;
    }
    this.style.background = checkedColor;
    checkedBtn = this;
  };
}
