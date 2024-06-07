#version 330 compatibility

uniform float uKa, uKd, uKs; 
uniform float uShininess; 

uniform float uAd, uBd;
uniform float uTol;
uniform float uNoiseAmp, uNoiseFreq;
uniform int uUseXYZforNoise;

uniform sampler3D Noise3;

in vec2 vST;
in vec3 vN; 
in vec3 vL; 
in vec3 vE; 
in vec3 vMCposition;

void
main( )
{
	vec3 Normal = normalize(vN);
	vec3 Light = normalize(vL);
	vec3 Eye = normalize(vE);
	vec3 myColor = vec3( 0.2, 0.7, 0.4 );		
	vec3 mySpecularColor = vec3( 0.8, 0.4, 0.9 );

	vec4 nv;
	if( uUseXYZforNoise == 1 )
			nv  = texture( Noise3, uNoiseFreq*vMCposition );
	else
			nv  = texture( Noise3, uNoiseFreq*vec3(vST,0.) );

	
	float n = nv.r + nv.g + nv.b + nv.a;
	n = n - 2.;
	n *= uNoiseAmp;

	float Ar = uAd/2.;
	float Br = uBd/2.;
	int numins = int( vST.s / uAd );
	int numint = int( vST.t / uBd );
	float sc = numins * uAd + Ar;
	float tc = numint * uBd + Br;
	float ds = vST.s - sc;
	float dt = vST.t - tc;
	
	float oldDist = sqrt( ds*ds + dt*dt );
	float newDist = oldDist + n;
	float scale = newDist / oldDist;

	ds *= scale;
	dt *= scale;
		
	ds /= Ar;
	dt /= Br;

	float smoothD = ds*ds + dt*dt;

	float t = smoothstep( 1. - uTol, 1. + uTol, smoothD );
	myColor = mix(vec3(0., 1., 0.4), vec3(0.2,0.7,0.4), t);
	
	vec3 ambient = uKa * myColor;
	float d = 0.;
	float s = 0.;
	if( dot(Normal,Light) > 0. ) 
	{
		d = dot(Normal,Light);
		vec3 ref = normalize( reflect( -Light, Normal ) ); 
		s = pow( max( dot(Eye,ref),0. ), uShininess );
	}
	vec3 diffuse =  uKd * d * myColor;
	vec3 specular = uKs * s * mySpecularColor;
	gl_FragColor = vec4( ambient + diffuse + specular, 1. );
}