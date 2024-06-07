#version 330 compatibility
varying vec4  vColor;
varying float vLightIntensity;
varying vec2  vST;
varying vec3  vXYZ;

out vec2 vST; // texture coords
out vec3 vN; // normal vector
out vec3 vL; // vector from point to light
out vec3 vE; // vector from point to eye
out vec3 vMCposition;

const vec3 LIGHTPOSITION = vec3( 5., 5., 0. );
uniform float A, B, C, D, Pi;

void main( )
{
    vST  = gl_MultiTexCoord0.st;
    vXYZ = gl_Vertex.xyz;

    vec3 tnorm = normalize( gl_NormalMatrix * gl_Normal );
    vec3 LightPos = vec3( 5., 10., 10. );
    vec3 ECposition = vec3( gl_ModelViewMatrix * gl_Vertex );
    vLightIntensity  = abs( dot( normalize(LightPos - ECposition), tnorm ) );
    
    if( vLightIntensity < 0.3 ) {
        vLightIntensity = 0.2;
    }

    vColor = gl_Color;
    if( gl_ProjectionMatrix[2][3] == 0. )
    vColor = vec4( 0.3, .5, 0.8, 1. );

	float y = gl_Vertex.y;
	float x = gl_Vertex.x;
	float r = sqrt(x*x + y*y);

	float drdx = x / r;
	float drdy = y / r;

	
	float dzdr = A * (-sin(2. * Pi * B * r + C) * 2. * Pi * B * exp(-D * r) + cos(2. * Pi * B * r + C) * -D * exp(-D*r));

	float dzdx = dzdr * drdx;
	float dzdy = dzdr * drdy;
	
	vec3 Tx = vec3(1., 0., dzdx);
	vec3 Ty = vec3(0., 1., dzdy);


	float z = A * cos(2. * Pi * B * r + C) * exp(-D * r);

	vec3 vert = gl_Vertex.xyz;
	vert.z = z;

	vST = gl_MultiTexCoord0.st;
	vMCposition = vert;
	vec4 ECposition = gl_ModelViewMatrix * vec4(vert, 1.);
	vN = normalize(cross(Tx, Ty));
	vL = LIGHTPOSITION - ECposition.xyz; 
	vE = vec3( 0., 0., 0. ) - ECposition.xyz; 
	gl_Position = gl_ModelViewProjectionMatrix * vec4(vert, 1.);
}
