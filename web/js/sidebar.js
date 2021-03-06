const sidebarApp = Vue.createApp({
  data() {
    return {
      checkedItem: null,
      btnItems: {
        homePanel: {
          checked: true,
          tipSee: false,
          tip: '主页',
          iconSrc: 'image/home.png',
          panel: document.getElementById('home-panel'),
          panelApp: homePanel,
          onclick: function () {},
        },
        protocolsPanel: {
          checked: false,
          tipSee: false,
          tip: '协议',
          iconSrc: 'image/blockchain.png',
          panel: document.getElementById('protocols-panel'),
          panelApp: protocolsPanel,
          onclick: function () {},
        },
      },
    };
  },

  methods: {
    _init() {
      var root = this;
      for (var key in root.btnItems) {
        var item = root.btnItems[key];
        item.onclick = function (nItem) {
          if (root.checkedItem === nItem) return;
          root.checkedItem.checked = false;
          root.checkedItem.panel.setAttribute('class', 'hidePanel');
          root.checkedItem = nItem;
          root.checkedItem.checked = true;
          root.checkedItem.panel.setAttribute('class', 'showPanel');
          topbarApp.checkedPanel = root.checkedItem.panelApp;

          topbarApp.setUpdateTime(
            root.checkedItem.panelApp,
            root.checkedItem.panelApp.updateTime,
            true
          );

          topbarApp.setUpPercent(
            root.checkedItem.panelApp,
            root.checkedItem.panelApp.upPrecent,
            true
          );
        };
      }
      root.checkedItem = root.btnItems.homePanel;
      topbarApp.checkedPanel = root.checkedItem.panelApp;
    },
  },

  mounted() {
    this._init();
  },
})
  .component('sidebar-item', {
    props: ['item'],
    template: `
    <div
          :class="[item.checked ? 'btnOnChecked' : 'btnOnUnchecked']"
          @click="item.onclick(item)"
          @mouseenter="item.tipSee = true"
          @mouseleave="item.tipSee = false"
    >
      <img :src="item.iconSrc" />
      <span v-if="item.tipSee">{{ item.tip }}</span>
    </div>
  `,
  })
  .mount('#sidebar');
