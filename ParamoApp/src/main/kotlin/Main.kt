import androidx.compose.desktop.ui.tooling.preview.Preview
import androidx.compose.runtime.Composable
import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import androidx.compose.ui.unit.dp

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color

@Composable
@Preview
fun App() {

    val uploadZoneWidth = 531.dp
    MaterialTheme(
        typography = Typography(
            defaultFontFamily = HelveticaFont
        ),
        colors = MaterialTheme.colors.copy(
            primary = Color(0xFF1DB3D6),
            secondary = Color(0xFF54C2BE)
        )
    ) {
        Box(
            modifier = Modifier.fillMaxSize().background(color = Color.White),
            contentAlignment = Alignment.BottomEnd
        ) {
            Column(
                modifier = Modifier.fillMaxSize(),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.Center
            ){
                AddFileZone(width = uploadZoneWidth)
                Spacer(modifier = Modifier.height(14.dp))
                AfterCompression(width = uploadZoneWidth)
            }

            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(80.dp)
                    .background(color = Color(0xFFFBFDFE))
                    .padding(end = 56.dp),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.End
            ) {
                Button(
                    onClick = {},
                    colors = ButtonDefaults.buttonColors(backgroundColor = Color.Transparent),
                    elevation = null
                ) {
                    Text("Cancel")
                }
                Button(
                    onClick = {},
                    elevation = null,
                    enabled = false,
                ) {
                    Text("Compress")
                }
            }
        }
    }
}

fun main() = application {
    Window(
        title = "Paramo App",
        onCloseRequest = ::exitApplication,
    ) {
        App()
    }
}
