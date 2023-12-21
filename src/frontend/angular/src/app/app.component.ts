import { Component } from '@angular/core';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass']
})
export class AppComponent {
  selectedProduct:string = ""
  title = 'graphql-example-ui';

  public show(productId:string){
      console.log(productId);
      this.selectedProduct= productId;
  }
}
