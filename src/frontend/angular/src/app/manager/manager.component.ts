import { HttpHeaders } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { Mutation } from 'apollo-angular';
import { Subscription } from 'rxjs';
import { AnyProductView, CancelMutationService, DeliverMutationService, GetManagerProductsQueryService , Restock, RestockMutationService} from '../models/generated-operations';

@Component({
  selector: 'app-manager',
  templateUrl: './manager.component.html',
  styleUrls: ['./manager.component.sass']
})
export class ManagerComponent implements OnInit {
  products: AnyProductView[] = [];
  loading: boolean = true;
  selectedId: string = "";

  private querySubscription!: Subscription;

  constructor(private getManagerProducts:GetManagerProductsQueryService,
      private deliver:DeliverMutationService,
      private cancel:CancelMutationService,
      private restock:RestockMutationService) {
  }
  ngOnDestroy(): void {
    this.querySubscription.unsubscribe();
  }

  public basketId(product:AnyProductView): string {
    if ( 'customerId' in product)
        return product.customerId!;
    else 
      return "";
  }

  public doAction(product:AnyProductView,action:string){
    let query! : Mutation;

    if ( action == "Deliver"){
      query=this.deliver;
    }
    if ( action == "Restock"){
      query=this.restock;
    }
    if ( action == "Cancel"){
      query= this.cancel;
    }
    if( query != null ){
      query.mutate(
        { productId: product.id }, {
        context: { headers: new HttpHeaders().set('managersecret', 'I`m Manager') },
        optimisticResponse:{
          __typename: "MutationType",
          response:{
            id: product.id,
            actionsAllowed:[ "Processing "],
          }
        },
        update:(cache,result,options)=>{
          if( action == "Restock"){
              const cacheId = cache.identify( (result.data as Restock).response! );
              console.log("r:",result,options,cacheId);
              cache.evict({ id: cacheId});
          }
        }
      }).subscribe((result)=>{
      });
   }
  }

  ngOnInit(): void {
    this.querySubscription =
      this.getManagerProducts.watch({},{
        errorPolicy: 'all',
        context: {
           headers: new HttpHeaders().set('managersecret', 'I`m Manager'),
        }
      })
        .valueChanges.subscribe({
          next: result => {
            console.log(result);
            this.products = result?.data?.allProducts as AnyProductView[];
            this.loading = result.loading;
          }, error: () => {
            this.loading = false;
          }
        });
  }

}
