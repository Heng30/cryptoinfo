const updateHomePanelItems = {
  data() {
    return {
      headerItems: [
        '...',
        '排名',
        '代币',
        '价格',
        '24h行情',
        '7d行情',
        '24h交易量',
      ],
      items: [],
      pdata: [],
      updateTime: new Date().toLocaleString(),
      upCoinPricePercent: 0,
    };
  },

  methods: {
    _loadItems() {
      var root = this;
      var url = serverUrl + '/apiv1/coin/private';
      chttp('GET', url, function (text) {
        try {
          var list = JSON.parse(text);
          if (!Array.isArray(list)) return;
          if (root.pdata.length <= 0) root._setChecked(list);
          root.pdata = list;
        } catch (e) {
          console.log(e);
        }
      });

      url = serverUrl + '/apiv1/coin/price';
      chttp('GET', url, function (text) {
        try {
          var upCount = 0;
          var list = JSON.parse(text);
          if (!Array.isArray(list)) return;
          list.forEach(function (value, index) {
            root._addItem(value, index);
            if (Number(value.percent_change_24h) > 0) upCount++;
          });

          root.upCoinPricePercent = (upCount * 100) / list.length;
          root.updateTime = new Date().toLocaleString();
        } catch (e) {
          console.log(e);
        }
      });
    },

    _addItem(value, index) {
      var items = [];
      items[0] = Number(index) + 1;
      items[1] = value.symbol;
      items[2] = toFixedPrice(value.price_usd);
      items[3] = toPercentString(value.percent_change_24h);
      items[4] = toPercentString(value.percent_change_7d);
      items[5] = toFixedPrice(value['24h_volume_usd']);

      var checked = false;
      this.pdata.forEach(function (v) {
        if (v.marked && v.symbol === value.symbol) checked = true;
      });

      this.items[index] = {
        info: items,
        isUpPercent: Number(value.percent_change_24h) >= 0,
        checked: checked,
      };
    },

    _setChecked(list) {
      this.items.forEach(function (item) {
        list.forEach(function (it) {
          if (it.symbol === item.info[1]) item.checked = true;
        });
      });
    },
  },

  mounted() {
    this._loadItems();
    setInterval(() => {
      this._loadItems();
    }, 10000);
  },
};

Vue.createApp(updateHomePanelItems)
  .component('home-panel-header', {
    props: ['items'],
    template: `
      <div>
        <p v-for="text in items" >{{ text }}</p>
      </div>
    `,
  })
  .component('home-panel-body-item', {
    props: ['items', 'checked'],
    template: `
    <div>
      <div>
        <div :class="[checked ? 'colorBtnChecked' : 'colorBtnUnchecked']"></div>
      </div>
        <p v-for="text in items">{{ text }}</p>
    </div>
  `,
  })
  .mount('#home-panel');

Vue.createApp(updateHomePanelItems).mount('#update-time');
Vue.createApp(updateHomePanelItems).mount('#coin-price-up-precent-24h');
