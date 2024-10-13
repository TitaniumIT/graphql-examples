import { Component, Input, OnChanges, OnInit, SimpleChanges } from '@angular/core';
import {  QueryRef } from 'apollo-angular';
import { Subscription } from 'rxjs';
import { CategoryView, GetProduct, GetProductQueryService, GetProductVariables, ProductDetailView } from '../models/generated-operations';

@Component({
  selector: 'app-product',
  templateUrl: './product.component.html',
  styleUrls: ['./product.component.sass']
})
export class ProductComponent implements OnInit,OnChanges {
  product!:ProductDetailView;
  categories:CategoryView[]|undefined = [];
  loading:boolean = true;
  
  private querySubscription!: Subscription;
  private query!: QueryRef<GetProduct, GetProductVariables>;

  @Input()  public productId:string = "-1";

  constructor(private getProduct:GetProductQueryService) { 
  }

  ngOnChanges(changes: SimpleChanges): void {
    if( this.query){
      this.loading = true;
      this.query.setVariables({ productId: this.productId,loadCategories: this.categories?.length==0 ?? true});
    }
  }

  ngOnDestroy(): void {
    this.querySubscription.unsubscribe();
  }

  ngOnInit(): void {
    this.query= this.getProduct.watch({ productId : this.productId , loadCategories: this.categories?.length==0 ?? true});
    this.querySubscription=this.query
      .valueChanges.subscribe(result => {
        this.product = result?.data?.product as ProductDetailView;
        if( result.data.categories){
          this.categories = result?.data.categories as CategoryView[]; 
        }
        this.loading = result.loading;
      });
  }

  public hasCategory(category:CategoryView):boolean{
    return this.product.selectedCategories?.some( x => x != null && x.id == category.id)!;
  }
}
