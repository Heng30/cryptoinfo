function IconBtn() {
  this.normalColor = 'lightgray';
  this.checkedColor = 'gray';
  this.checkedBtn = null;
  this.iconBtns = ['home-btn', 'protocol-btn'];
  this.panels = ['home-panel', 'protocol-panel'];
  this.init = function () {
    var root = this;
    for (var i = 0; i < root.iconBtns.length; i++) {
      var btn = document.getElementsByClassName(root.iconBtns[i])[0];
      var panel = document.getElementById(root.panels[i]);
      btn.associatePanel = panel;

      btn.onclick = function () {
        if (root.checkedBtn === this) return;
        root.checkedBtn.style.background = root.normalColor;
        root.checkedBtn.associatePanel.style.display = 'none';

        this.style.background = root.checkedColor;
        this.associatePanel.style.display = 'flex';
        root.checkedBtn = this;
      };

      if (i === 0) {
        panel.style.display = 'flex';
        btn.style.background = root.checkedColor;
        root.checkedBtn = btn;
      } else {
        panel.style.display = 'none';
      }
    }
  };
}
