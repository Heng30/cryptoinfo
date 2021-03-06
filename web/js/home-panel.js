const homePanel = Vue.createApp({
  data() {
    return {
      name: 'home',
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
      updateTime: null,
      upPrecent: -1,
      isUpSort: false,
      sortIndex: 0,
      sortItems: function () {},
      headerItemIndex: function () {},
    };
  },

  methods: {
    _headerItemIndex(text) {
      for (var i = 0; i < this.headerItems.length; i++) {
        if (text === this.headerItems[i]) return i;
      }
      return 0;
    },
    _sortItems(index, isTimeout) {
      var root = this;
      if (root.items.length <= 0) return;
      if (index === root.sortIndex) {
        if (!isTimeout) root.isUpSort = !root.isUpSort;
      } else {
        root.sortIndex = index;
      }

      var isUpSort = root.isUpSort;
      this.items.sort(function (a, b) {
        if (!isUpSort) [a, b] = [b, a];
        if (index === 0) {
          if (a.checked < b.checked) return -1;
          else if (a.checked === b.checked) return 0;
          else return 1;
        } else {
          if (a.rawInfo[index - 1] < b.rawInfo[index - 1]) return -1;
          else if (a.rawInfo[index - 1] === b.rawInfo[index - 1]) return 0;
          else return 1;
        }
      });

      root.items.forEach(function (value, index) {
        var items = value.rawInfo.slice(0);
        items[3] = toPercentString(items[3]);
        items[4] = toPercentString(items[4]);
        items[5] = toFixedPrice(items[5]);
        root.items[index].info = items;
      });
    },
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

          root._sortItems(root.sortIndex, true);

          root.updateTime = new Date().format('hh:mm:ss');
          root.upPrecent = ((upCount * 100) / list.length).toFixed(2);
          topbarApp.setUpdateTime(root, root.updateTime);
          topbarApp.setUpPercent(root, root.upPrecent);
        } catch (e) {
          console.log(e);
        }
      });
    },

    _addItem(value, index) {
      var items = [];
      items[0] = Number(index) + 1;
      items[1] = value.symbol;
      items[2] = Number(toFixedPrice(value.price_usd));
      items[3] = Number(value.percent_change_24h);
      items[4] = Number(value.percent_change_7d);
      items[5] = Number(value['24h_volume_usd']);

      var checked = false;
      this.pdata.forEach(function (v) {
        if (v.marked && v.symbol === value.symbol) checked = true;
      });

      this.items[index] = {
        rawInfo: items,
        info: [],
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
    this.sortItems = this._sortItems;
    this.headerItemIndex = this._headerItemIndex;
    this._loadItems();

    setInterval(() => {
      this._loadItems();
    }, 10000);
  },
})
  .component('home-panel-header', {
    props: ['items', 'sort_items', 'item_index'],
    template: `
      <div>
        <button
            v-for="text in items"
            @click="sort_items(item_index(text))"
        >{{ text }}</button>
      </div>
    `,
  })
  .component('home-panel-body-item', {
    props: ['items', 'checked'],
    template: `
    <div>
      <div>
        <span :class="[checked ? 'colorBtnChecked' : 'colorBtnUnchecked']"></span>
      </div>
        <p v-for="text in items">{{ text }}</p>
    </div>
  `,
  })
  .mount('#home-panel');
