import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AppComponent } from './app.component';
import { ClientComponent } from './client/client.component';
import { ManagerComponent } from './manager/manager.component';

const routes: Routes = [
  {
    path: "manager",
    component: ManagerComponent
  },
  {
    path: "",
    component: ClientComponent
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
