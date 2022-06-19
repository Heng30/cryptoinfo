 function HomePanel() {
   this.homePanelBody = null;
   this.init = function () {
     this.homePanelBody = document.getElementById('home-panel-body');

     for (var i = 0; i < 100; i++) {
       this.newItem(i);
     }

     this.refresh();
   };

   this.newItem = function (index) {
     var div = document.createElement('div');
     div.setAttribute('id', 'home-panel-body-item' + index);

     for (var i = 0; i < 6; i++) {
       var p = document.createElement('p');
       p.textContent = i;
       div.appendChild(p);
     }
     this.homePanelBody.appendChild(div);
   };

   this.removeItem = function (count) {
     var children = this.homePanelBody.children;
     for (var i = 0; i < count && 0 < children.length; i++)
       children[children.length - 1].remove();
   };

   this.update = function () {
     if (!this.homePanelBody) return;
     var children = this.homePanelBody.children;
     for (var i = 0; i < children.length; i++) {
       var div = children[i];
       var pTags = div.children;
       for (var j = 0; j < pTags.length; j++) {
         pTags[j].textContent = (Math.random() * 100).toFixed(0);
       }
     }
   };

   this.refresh = function () {
     var root = this;
     setInterval(function () {
       root.update();
     }, 1000);
   };
 }

