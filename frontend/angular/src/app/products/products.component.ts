import { Component, EventEmitter, OnDestroy, OnInit, Output } from '@angular/core';
import { Subscription } from 'rxjs';
import { BuyProductMutationService,  GetProductsQueryService, ProductView,PageInfoView, GetProducts, GetProductsVariables } from '../models/generated-operations';
import { Apollo, QueryRef } from 'apollo-angular';
import { BasketService } from '../services/basket-service.service';
import { PageInfoFieldPolicy } from '../models/generated-clientHelpers';

@Component({
  selector: 'app-products',
  templateUrl: './products.component.html',
  styleUrls: ['./products.component.sass']
})
export class ProductsComponent implements OnInit, OnDestroy {
  products: ProductView[] = [];
  pageInfo: PageInfoView = {hasNextPage:false,hasPreviousPage:false};
  loading: boolean = true;
  selectedId: string = "";

  private querySubscription!: Subscription;
  private queryRef! : QueryRef<GetProducts, GetProductsVariables>;

  @Output() selectedProductId = new EventEmitter<string>()

  constructor(private getProducts:GetProductsQueryService,private basket:BasketService ) {
  }
  ngOnDestroy(): void {
    this.querySubscription.unsubscribe();
  }

  ngOnInit(): void {

    this.queryRef= this.getProducts.watch();

    this.querySubscription = this.queryRef.valueChanges.subscribe({
          next: result => {
            console.log("products:", result);
            this.products = result?.data?.productsRelay?.edges?.map( e => e?.node as ProductView) as ProductView[];
            this.pageInfo = result?.data?.productsRelay?.pageInfo as PageInfoView;
            this.loading = result.loading;
          }, error: () => {
            this.loading = false;
          }
        });
  }

  public NextPage(){
    const vars =  {
      first: 5,
      after: this.pageInfo.endCursor,
      last:null,
      before:null
    };
     this.queryRef.fetchMore({
        variables: vars,
      }).then( ()=>this.ResetSelection());
  }

  public PreviousPage(){
    const vars =  {
      first:null,
      after:null,
      last: 5,
      before: this.pageInfo.startCursor
    };
     this.queryRef.fetchMore({
        variables: vars,
      }).then( ()=>this.ResetSelection());
  }

  
  public ResetSelection() {
    this.selectedProductId.emit("");
    this.selectedId = "";
  }

  public Select(product: ProductView) {
    this.selectedProductId.emit(product.id);
    this.selectedId = product.id;
  }

  public Buy(product: ProductView): void {
    console.log("Ordering");
    this.basket.buyProduct(product);
    
}
}