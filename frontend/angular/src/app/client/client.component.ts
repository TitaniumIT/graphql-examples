import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-client',
  templateUrl: './client.component.html',
  styleUrls: ['./client.component.sass']
})
export class ClientComponent implements OnInit {
  selectedProduct:string = ""

  constructor() { }

  ngOnInit(): void {
  }

  public show(productId:string){
      this.selectedProduct= productId;
  }
}
