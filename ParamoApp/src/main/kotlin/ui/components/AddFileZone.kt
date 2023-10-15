import androidx.compose.foundation.Image
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.OutlinedButton
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

@Composable
fun AddFileZone(width: Dp){
    Column(
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally,
        modifier = Modifier.border(
            width = 1.dp,
            color = Color(0x40000000),
            shape = RoundedCornerShape(size = 10.dp),
        )
            .padding(1.dp)
            .width(width)
            .height(284.dp)
    ) {
        Image(
            painter = painterResource("image/upload.png"),
            contentDescription = "Upload Icon",
            modifier = Modifier.size(48.dp)
        )
        Spacer(modifier = Modifier.height(24.dp))

        Column(
            modifier = Modifier.width(252.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Text(
                text = "Select a file or drag and drop here",
                fontSize = 13.sp,
            )
            Spacer(modifier = Modifier.height(12.dp))
            Text(
                text = "Do not attempt to compress already compressed file types like MP3, MP4 or archives.",
                fontSize = 13.sp,
                color = Color(0x66000000),
                textAlign = TextAlign.Center
            )
            Spacer(modifier = Modifier.height(24.dp))
            OutlinedButton(
                onClick = {}
            ){
                Text(
                    "Select File".uppercase(),
                    fontSize = 10.sp,
                )
            }
        }
    }
}