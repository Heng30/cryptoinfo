const topbarApp = Vue.createApp({
  data() {
    return {
      checkedPanel: null,
      updateTime: null,
      upPercent: -1,
    };
  },
  methods: {
    setUpdateTime(panel, time, isClick) {
      if (panel === this.checkedPanel) this.updateTime = time;
      else if (isClick) this.updateTime = null;
    },
    setUpPercent(panel, precent, isClick) {
      if (panel === this.checkedPanel) this.upPercent = precent;
      else if (isClick) this.upPercent = -1;
    },
    _loadItems() {},
  },
  mounted() {
    this._loadItems();
    setInterval(() => {
      this._loadItems();
    }, 60000);
  },
}).mount('#top-bar');
