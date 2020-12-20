#if 0
#include<bits/stdc++.h>
using namespace std;
using Int = long long;
template<typename T1,typename T2> inline void chmin(T1 &a,T2 b){if(a>b) a=b;}
template<typename T1,typename T2> inline void chmax(T1 &a,T2 b){if(a<b) a=b;}

//INSERT ABOVE HERE
const Int MAX = 305;
const Int MAX2 = MAX*2;
signed ex[MAX2][MAX2];
signed dp1[MAX2+2][MAX2+2];
signed dp2[MAX2+2][MAX2+2];
int main(){
  Int h,w;
  cin>>h>>w;
  vector<string> s(h);
  for(Int i=0;i<h;i++) cin>>s[i];

  using P = pair<Int, Int>;
  vector<P> vp;
  memset(ex,0,sizeof(ex));
  for(Int i=0;i<h;i++){
    for(Int j=0;j<w;j++){
      if(s[i][j]!='#') continue;
      vp.emplace_back(1+i+j,1+i-j+w);
      ex[1+i+j][1+i-j+w]=1;      
    }
  }
  
  auto ch=
    [&](Int i,Int j)->Int{
      if(i<0||i>=MAX2) return 0;
      if(j<0||j>=MAX2) return 0;
      return ex[i][j];
    };
  
  memset(dp1,0,sizeof(dp1));
  memset(dp2,0,sizeof(dp2));
  Int ans=0;
  
  const Int LIM = h+w;
  for(Int k=1;k<=LIM;k++){
    for(Int i=0;i<MAX2;i++){
      for(Int j=0;j<MAX2;j++){
        dp1[i][j+1]=dp1[i][j]+(ex[i][j]&&ch(i,j-k));
        dp2[i][j+1]=dp2[i][j]+(ex[j][i]&&ch(j-k,i));
      }
    }
    for(P p:vp){
      Int i,j;
      tie(i,j)=p;
      
      if(i-k>=0) ans+=dp1[i-k][min(MAX2,j+k+1)]-dp1[i-k][j];
      if(j-k>=0) ans+=dp2[j-k][min(MAX2,i+k+1)]-dp2[j-k][i];
      
      if(i+k<MAX2) ans+=dp1[i+k][min(MAX2,j+k+1)]-dp1[i+k][j];
      if(j+k<MAX2) ans+=dp2[j+k][min(MAX2,i+k+1)]-dp2[j+k][i];
    }            
  }

  //cout<<ans<<endl;
  for(P p:vp){
    Int i,j;
    tie(i,j)=p;
    for(Int k=1;k<=LIM;k++){
      if(ch(i+k,j)&&ch(i,j+k)) ans--;
      if(ch(i+k,j)&&ch(i,j-k)) ans--;
      if(ch(i-k,j)&&ch(i,j+k)) ans--;
      if(ch(i-k,j)&&ch(i,j-k)) ans--;
    }
  }
  cout<<ans<<endl;
  return 0;
}

#else

#include<cstdio>
#include<algorithm>
#include<cstring>
#define rep(i,l,r) for (int i=(l); i<=(r); i++)
using namespace std;

const int N=610;
struct point{ int x,y; }p[90010];
int h,w,m=300,cnt,a[N][N],r[N][N],c[N][N];
long long ans;

int main(){
	scanf("%d%d",&h,&w);
	rep(i,1,h){
		char str[310]; scanf("%s",str+1);
		rep(j,1,w){
			if (str[j]=='#'){
				int x=i+j-1,y=i-j+m;
				p[cnt++]=(point){x,y}; a[x][y]=1;
			}
		}
	}
	m*=2;
	rep(i,1,m) rep(j,1,m) r[i][j]=r[i][j-1]+a[i][j],c[i][j]=c[i-1][j]+a[i][j];
	for (int i=0;i<cnt;i++){
		int x=p[i].x,y=p[i].y;
		for (int d=2;d<=m;d+=2){
			if (y+d<=m && a[x][y+d]){
				if (x+d<=m) ans+=r[x+d][y+d]-r[x+d][y-1];
				if (x-d>0) ans+=r[x-d][y+d]-r[x-d][y-1];
			}
			if (x+d<=m && a[x+d][y]){
				if (y+d<=m) ans+=c[x+d-1][y+d]-c[x][y+d];
				if (y-d>0) ans+=c[x+d-1][y-d]-c[x][y-d];
			}
		}
	}
	printf("%lld\n",ans);
	return 0;
}

#endif