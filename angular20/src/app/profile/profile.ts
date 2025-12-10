import { NgOptimizedImage } from '@angular/common';
import { Component, afterNextRender, NgZone, ChangeDetectorRef, OnInit,signal } from '@angular/core';
import { Profileservice } from '../services/profileservice';
import { SessionStorage } from '../services/session-storage';
import { FormGroup, FormControl, Validators, ReactiveFormsModule } from '@angular/forms';
import { Footer } from '../footer/footer';
declare var $: any;

@Component({
  selector: 'app-profile',
  imports: [NgOptimizedImage,ReactiveFormsModule,Footer],
  templateUrl: './profile.html',
  styleUrl: './profile.scss'
})

export class Profile implements OnInit  {  
  profileMsg = signal('');

  passwordChangeForm = new FormGroup({
    newpassword: new FormControl('', Validators.required),
    confnewpassword: new FormControl('', [Validators.required])
  });

  profileForm = new FormGroup({
    firstname: new FormControl('', Validators.required),
    lastname: new FormControl('', Validators.required),
    mobile: new FormControl('', Validators.required)
  });

  profileData: any;
  userId: any;
  jwttoken: any;
  enableMfa: any = [];
  mfa: boolean = false;
  profilepic: string = '';
  email: any = '';
  qrcodeurl: any = null;
  userpicture: string = '';
  showSave: boolean = false;

  constructor(
    private profileService: Profileservice,
    private sessionStorageSevice: SessionStorage,
    private ngZone: NgZone, 
    private cdRef: ChangeDetectorRef    
  ) 
  {
    afterNextRender(() => {
      console.log('Window object is safe to use here:', window.location.href);
    });
  
    const uid = this.sessionStorageSevice.getItem('USERID');
    if (uid) {
     this.userId = uid;
    }

    const jwt = this.sessionStorageSevice.getItem('TOKEN');
    if (jwt) {
     this.jwttoken = jwt;
    }
    this.profileMsg.set("retrieving records...");
    this.profileService.getUserbyId(this.userId, this.jwttoken).subscribe({
     next: (res: any) => {
           this.profileMsg.set(res.message);
           const formData = {
             firstname: res.firstname,
             lastname: res.lastname,
             mobile: res.mobile
            }
            this.profileForm.setValue(formData);
            $("#email").val(res.email);
            let userpicture = `http://127.0.1.:3000/assets/users/${res.userpic}`
            this.profilepic = userpicture;
            if (res.qrcodeurl !== "") {
              let qrcode: any = `data:image/png;base64,${res.qrcodeurl}`;
              this.qrcodeurl = qrcode;  
              this.mfa = true;
            } else {
              this.mfa = false;
              let qrcode: any = "http://127.0.0.1:3000/assets/images/qrcode.png";
              this.qrcodeurl = qrcode;
            }
            setTimeout(() => {
              this.profileMsg.set('');
            }, 1000);
   
       },
       error: (err: any) => {
         this.profileMsg.set(err.error.message);
         setTimeout(() => {
           this.profileMsg.set('');
         }, 3000);

       }
    });        
  }

  ngOnInit(): void {
    $("#cpwd").hide();
    $("#mfa1").hide();
    $("#mfa2").hide();  
  }


  changeProfilepic(event: any) {
    $("#pix").attr('src',URL.createObjectURL(event.target.files[0]));
    
    const formdata = new FormData();
    formdata.append('userpic',event.target.files[0]);
    this.profileService.UploadPicture(this.userId ,formdata, this.jwttoken).subscribe({
      next: (res: any) => {
        this.profileMsg.set(res.message);
        $('#twofactor').prop('checked', false);
        $('#changepwd').prop('checked', false);      
        setTimeout(() => {
          let userpicture = `http://127.0.1.:3000/assets/users/${res.userpic}`
          this.profilepic = userpicture;
          this.sessionStorageSevice.setItem('USERPIC', userpicture);
          this.profileMsg.set('');
          window.location.reload();
        }, 3000);
      },
      error: (err: any) => {

          this.profileMsg.set(err.error.message);
          setTimeout(() => {
            this.profileMsg.set('');
          }, 3000);

      }

    });    
  }

  changePassword() {
    if ($('#changepwd').is(":checked")) {
      this.showSave = true;
      $("#cpwd").show();
      $("#mfa1").hide();  
      $("#mfa2").hide();  
      $('#twofactor').prop('checked', false);
    } else {
      this.showSave = false;
      $("#cpwd").hide();
    }            
  }

  onetimePassword() {
    if ($('#twofactor').is(":checked")) {
      this.showSave = true;
      $("#cpwd").hide();
      $("#mfa1").show();
      $("#mfa2").show();
      $('#changepwd').prop('checked', false);
    } else {
      $("#mfa1").hide();  
      $("#mfa2").hide();  
      this.showSave = false;
    }            
  }

  passwordChange() {
    this.ngZone.run(() => {
      if (this.passwordChangeForm.get('newpassword')?.value === '') {
        this.profileMsg.set('Please enter New password...')
        setTimeout(() => {
          this.profileMsg.set('');
        }, 3000)
        return;
      }

      if (this.passwordChangeForm.get('confnewpassword')?.value === '') {
        this.profileMsg.set('Please confirm New password...')
        setTimeout(() => {
          this.profileMsg.set('');
        }, 3000)
        return;
      }

      if (this.passwordChangeForm.get('newpassword')?.value != this.passwordChangeForm.get('confnewpassword')?.value) {
        this.profileMsg.set('New password does not mactched...')
        setTimeout(() => {
          this.profileMsg.set('');
        }, 3000)
        return;
      }
      const formData = {
        'password': this.passwordChangeForm.get('newpassword')?.value
      }
      
      this.profileService.sendNewpasswordRequest(this.userId, formData, this.jwttoken).subscribe({
        next: (res: any) => {

          this.profileMsg.set(res.message);
          setTimeout(() => {
            this.profileMsg.set('');
          }, 3000);

      },
      error: (err: any) => {
        this.profileMsg.set(err.error.message);
        setTimeout(() => {
          this.profileMsg.set('');
        }, 3000);

      }

    });      

    }); //END-ngZone
  }

  enableMFA(event: any) {
    event.preventDefault();    
    this.enableMfa = {
      TwoFactorEnabled: true
    }

    this.profileService.ActivateMFA(this.userId, this.enableMfa, this.jwttoken).subscribe({
      next: (res: any) => {
        
          this.profileMsg.set(res.message);
          setTimeout(() => {
            this.profileMsg.set('');
            this.mfa = true;
            let qrcode: any = 'data:image/png;base64,'+res.qrcodeurl;
            this.qrcodeurl = qrcode;
          }, 3000);

        },
        error: (err: any) => {

          this.profileMsg.set(err.error.message);
          setTimeout(() => {
            this.profileMsg.set('');
            this.qrcodeurl = null;
          }, 3000);
  
        }  
    });
  }

  disableMFA(event: any) {
    event.preventDefault();      
    this.enableMfa = {
      TwoFactorEnabled: false
    }

    this.profileService.ActivateMFA(this.userId, this.enableMfa, this.jwttoken).subscribe({
      next: (res: any) => {

        this.profileMsg.set(res.message);
        let qrcode: any = 'http://127.0.0.1:3000/assets/images/qrcode.png';
        this.qrcodeurl = qrcode ;

      },
      error: (err: any) => {

        this.profileMsg.set(err.error.message);
        setTimeout(() => {
          this.profileMsg.set('');
        }, 3000);

      }

    });
    setTimeout(() => {
      this.profileMsg.set('');
    }, 3000);

  }

  submitProfileForm() {
    this.ngZone.run(() => {
        this.profileMsg.set("please wait..");
        const jsonData = { 
          'firstname': this.profileForm.get('firstname')?.value,
          'lastname': this.profileForm.get('lastname')?.value, 
          'mobile': this.profileForm.get('mobile')?.value};
        this.profileService.sendProfileRequest(this.userId,jsonData, this.jwttoken).subscribe({
          next: (res: any) => {
            this.profileMsg.set(res.message);
            setTimeout(() => {
              this.profileMsg.set('');
            }, 3000);

          },
          error: (err: any) => {
            this.profileMsg.set(err.error.message);
            setTimeout(() => {
              this.profileMsg.set('');
            }, 3000);

          }
          
      });      
    });
  }  


}
