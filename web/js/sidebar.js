const sidebar = {
  data() {
    return {
      checkedItem: null,
      btnItems: {
        homePanel: {
          checked: true,
          iconSrc: 'image/home.png',
          panel: document.getElementById('home-panel'),
          onclick: function () {},
        },
        protocolPanel: {
          checked: false,
          iconSrc: 'image/home.png',
          panel: document.getElementById('protocol-panel'),
          onclick: function () {},
        },
      },
    };
  },

  methods: {
    _init() {
      var root = this;
      this.checkedItem = this.btnItems.homePanel;
      for (var key in this.btnItems) {
        var item = this.btnItems[key];
        item.onclick = function (nItem) {
          if (root.checkedItem === nItem) return;
          root.checkedItem.checked = false;
          root.checkedItem.panel.setAttribute("class", "hidePanel")
          root.checkedItem = nItem;
          root.checkedItem.checked = true;
          root.checkedItem.panel.setAttribute("class", "showPanel")
        };
      }
    },
  },

  mounted() {
    this._init();
  },
};

Vue.createApp(sidebar).mount('#sidebar');
