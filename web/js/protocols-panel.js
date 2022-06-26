const protocolsPanel = Vue.createApp({
  data() {
    return {
      name: 'protocols',
      headerItems: [
        '排名',
        '名称',
        '代币',
        '锁仓量',
        '质押',
        '市值',
        '24h行情',
        '7d行情',
      ],
      items: [],
      updateTime: null,
      upPercent: -1,
      isUpSort: true,
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
        if (!isTimeout) this.isUpSort = !root.isUpSort;
      } else {
        root.sortIndex = index;
      }

      var isUpSort = root.isUpSort;
      this.items.sort(function (a, b) {
        if (!isUpSort) [a, b] = [b, a];
        if (a.rawInfo[index] < b.rawInfo[index]) return -1;
        else if (a.rawInfo[index] === b.rawInfo[index]) return 0;
        else return 1;
      });

      root.items.forEach(function (value, index) {
        var items = value.rawInfo.slice(0);
        items[3] = items[3] > 0 ? toFixedPrice(items[3]) : 'N/A';
        items[4] = items[4] > 0 ? toFixedPrice(items[4]) : 'N/A';
        items[5] = items[5] > 0 ? toFixedPrice(items[5]) : 'N/A';
        items[6] = toPercentString(items[6]);
        items[7] = toPercentString(items[7]);
        root.items[index].info = items;
      });
    },
    _loadItems() {
      var root = this;
      const url = serverUrl + '/apiv1/defi/protocols';
      chttp('GET', url, function (text) {
        try {
          var upCount = 0;
          var list = JSON.parse(text);
          const maxCount = 100;
          if (!Array.isArray(list)) return;
          for (var i = 0; i < list.length; i++) {
            if (i >= maxCount) break;
            var value = list[i];
            root._addItem(value, i);
            if (Number(value.change_1d) > 0) upCount++;
          }

          root._sortItems(root.sortIndex, true);
          root.updateTime = new Date().format('hh:mm:ss');
          root.upPrecent = ((upCount * 100) / maxCount).toFixed(2);
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
      items[1] = value.name.replaceAll(' ', '').replaceAll('&', '');
      items[2] = value.symbol;
      items[3] = value.tvl ? Number(value.tvl) : -1;
      items[4] = value.staking ? Number(value.staking) : -1;
      items[5] = value.mcap ? Number(value.mcap) : -1;
      items[6] = Number(value.change_1d);
      items[7] = Number(value.change_7d);

      this.items[index] = {
        rawInfo: items,
        info: [],
        isUpPercent: Number(value.change_1d) >= 0,
      };
    },
  },

  mounted() {
    this.sortItems = this._sortItems;
    this.headerItemIndex = this._headerItemIndex;
    this._loadItems();

    setInterval(() => {
      this._loadItems();
    }, 1000 * 600);
  },
})
  .component('protocols-panel-header', {
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
  .component('protocols-panel-body-item', {
    props: ['items'],
    template: `
    <div>
        <p v-for="text in items">{{ text }}</p>
    </div>
  `,
  })
  .mount('#protocols-panel');
