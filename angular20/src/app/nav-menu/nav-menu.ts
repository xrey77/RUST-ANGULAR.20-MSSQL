import { NgbModal, NgbModalConfig } from '@ng-bootstrap/ng-bootstrap';
import { Component, OnInit, afterNextRender, ViewChild, TemplateRef } from '@angular/core';
import { RouterLinkWithHref } from '@angular/router';
import { SessionStorage } from '../services/session-storage';
import { Login } from "../login/login";
import { Register } from "../register/register";
import { Logoutservice } from '../services/logoutservice';
import { NgbDropdownModule, NgbNavModule } from '@ng-bootstrap/ng-bootstrap';
import { Router } from '@angular/router';

declare var $: any;

@Component({
  selector: 'app-nav-menu',
  imports: [RouterLinkWithHref,NgbNavModule, 
    NgbDropdownModule,Login, Register], 
  templateUrl: './nav-menu.html',
  styleUrl: './nav-menu.scss',
	providers: [NgbModalConfig, NgbModal],  
})

export class NavMenu implements OnInit {
    userName: string = '';
    profilepic: string = '';
    loginContent: any;
    loginTemplate: any;
    content: TemplateRef<any>|null|undefined;
  
  constructor(
    private router: Router,    
    private sessionStorageService: SessionStorage,
    private logoutService: Logoutservice
    ) { 
        afterNextRender(() => {
          // This code runs only in the browser, after the next render cycle
          console.log('Window object is safe to use here:', window.location.href);
        });    
    }
  
    ngOnInit(): void {
        try {
        const uname = this.sessionStorageService.getItem('USERNAME');
        if (uname) {
          this.userName = uname;
        }
      } catch(error) {}

      try {
        const userpic = this.sessionStorageService.getItem('USERPIC');
        if (userpic) {
          this.profilepic = userpic;
        }
      } catch(error) {}
    }
  
    goToHome() {
      this.router.navigate(['/']); 
    }
  
  
  logOut(){
    this.sessionStorageService.removeItem('USERNAME');
    this.sessionStorageService.removeItem('USERPIC');
    this.sessionStorageService.removeItem('USERID');
    this.sessionStorageService.removeItem('TOKEN');
    this.sessionStorageService.clear();
    setTimeout(() => {
      this.goToHome();
      location.href="/";
    }, 800);

  }

  userloginOpen() {
    $("#showLogin").trigger('click');
	}

  userRegistrationOpen() {
    $("#showRegistration").trigger('click');
  }

}
