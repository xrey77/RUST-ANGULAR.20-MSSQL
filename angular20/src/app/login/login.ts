import { NgbModal, NgbModalConfig } from '@ng-bootstrap/ng-bootstrap';
import { Component, signal, afterNextRender,Input, TemplateRef, AfterViewInit } from '@angular/core';
import { Mfa } from "../mfa/mfa";
import { Loginservice } from '../services/loginservice';
import { Logoutservice } from '../services/logoutservice';
import { SessionStorage } from '../services/session-storage';
import { FormGroup, FormControl, Validators, ReactiveFormsModule } from '@angular/forms';
import { Router } from '@angular/router';

declare var $: any;

@Component({
  selector: 'app-login',
  imports: [Mfa, ReactiveFormsModule],
  templateUrl: './login.html',
  styleUrl: './login.scss',
	providers: [NgbModalConfig, NgbModal],  
})

export class Login implements AfterViewInit{
  @Input() templateRef?: TemplateRef<any>;

  loginMessage = signal('');
  isDisabled: boolean = false;

  loginForm = new FormGroup({
    username: new FormControl('', [Validators.required]),
    password: new FormControl('', [Validators.required])
  });


  constructor(  
    private router: Router,
    config: NgbModalConfig,
		private modalService: NgbModal,
    private loginService: Loginservice,
    private logoutService: Logoutservice,
    private sessionStorageService: SessionStorage
  ) { 
    config.backdrop = 'static';
		config.keyboard = false;    
      afterNextRender(() => {
        // This code runs only in the browser, after the next render cycle
        console.log('Window object is safe to use here:', window.location.href);
      });  
  }
  ngAfterViewInit(): void {
  }

  submitLoginForm() {
    if(this.loginForm.valid)
    {
       this.loginMessage.set('please wait...');
       this.isDisabled = true;
       this.loginService.sendLoginRequest(this.loginForm.value).subscribe({
         next: (res: any) => {
            if (res.isactivated == 0) {
              this.loginMessage.set('Please activate your account first.')
              setTimeout(() => {
                this.loginMessage.set('')
                this.isDisabled = false;
              }, 3000);  
              return;
            }
            if (res.isblocked === '1') {
              this.loginMessage.set('You Account has been blocked.')
              setTimeout(() => {
                this.loginMessage.set('')
                this.isDisabled = false;
              }, 3000);  
              return;
            }

            this.loginMessage.set(res.message);


            this.sessionStorageService.setItem("USERID", res.id);
            this.sessionStorageService.setItem("TOKEN", res.token);
            let userpicture = `http://127.0.1.:3000/assets/users/${res.userpic}`
            this.sessionStorageService.setItem("USERPIC", userpicture);

            if (res.qrcodeurl !== "") {
              this.isDisabled = false;
              $("#reset").trigger('click'); 
              $("#hideLogin").trigger('click');
              $("#showMfa").trigger('click');
              this.loginMessage.set('');
              return;
            } else {
              this.sessionStorageService.setItem("USERNAME", res.username);  
              this.loginMessage.set('');
              $("#reset").trigger('click');
              $("#hideLogin").trigger('click');
              setTimeout(() => {
                this.goToHome();
                location.href="/";
              }, 800);
            }
          },
          error: (err: any) => {
            this.loginMessage.set(err.error.message);
            setTimeout(() => {
              this.loginMessage.set('');
              this.isDisabled = false;
            }, 3000);

          }

      });
    }
  }

  public loginOpen(loginTemplate: any): void {
		this.modalService.open(loginTemplate, { size: 'sm', centered: true });
	}

  goToHome() {
    this.router.navigate(['/']); 
  }

  closeLogin() {
    $("#reset").trigger('click');
    this.goToHome;
    location.reload();
  }

}
